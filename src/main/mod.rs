//! MontionCtrl
//!
//!

mod stepprofile;

use self::stepprofile::{Num, StepProfile};
use crate::interfaces::{DelayToTicksTrait, MotionControlStepModeTrait, MotionControlTrait};
use crate::SetDirectionTrait;

use super::{Direction, ResetTrait, SetStepModeTrait, StepTrait};
// use core::{convert::TryFrom, ops};
use ramp_maker::MotionProfile;

pub struct MontionCtrl<DRIVER, Convert> {
    // state: State<Driver, Timer, Profile>,
    driver: DRIVER,
    current_step: i32,
    current_direction: Direction,
    convert: Convert,
}

impl<DRIVER, Convert> MontionCtrl<DRIVER, Convert> {
    pub fn new(driver: DRIVER, convert: Convert ) -> Self {
        Self {
            driver,
            current_step: 0,
            current_direction: Direction::Forward,
            convert,
        }
    }

    pub fn release(self) -> Result<(DRIVER,), ()> {
        Ok((self.driver,))
    }
}

impl<DRIVER, Convert> MotionControlStepModeTrait for MontionCtrl<DRIVER, Convert>
where
    DRIVER: SetStepModeTrait + ResetTrait,
    DRIVER::StepMode: Copy,
    Convert: DelayToTicksTrait,
{
    type StepMode = DRIVER::StepMode;

    /// Set step mode of the wrapped driver
    ///
    fn set_step_mode(&mut self, step_mode: Self::StepMode) -> Result<(), ()> {
        let mut do_modify = || match self.driver.apply_mode_config(step_mode) {
            Err(_) => return Err(()),
            Ok(_) => return Ok(()),
        };

        let total = DRIVER::SETUP_TIME + DRIVER::HOLD_TIME;
        match self.convert.wait(&total, || {
            return do_modify();
        }) {
            Ok(_) => {
                let total = DRIVER::RESET_SETUP_TIME + DRIVER::RESET_HOLD_TIME;
                let do_enable = || match self.driver.enable_driver() {
                    Err(_) => return Err(()),
                    Ok(_) => return Ok(()),
                };
                return self.convert.wait(&total, do_enable);
            }
            Err(_) => return Err(()),
        }
    }
}

impl<DRIVER, Convert> MotionControlTrait for MontionCtrl<DRIVER, Convert>
where
    DRIVER: SetDirectionTrait + StepTrait,
    Convert: DelayToTicksTrait,
{
    fn set_direction(&mut self, direction: Direction) -> Result<(), ()> {
        let do_modify = || match direction {
            Direction::Forward => match self.driver.set_forward() {
                Err(_) => return Err(()),
                Ok(_) => return Ok(()),
            },
            Direction::Backward => match self.driver.set_backward() {
                Err(_) => return Err(()),
                Ok(_) => return Ok(()),
            },
        };

        match self.convert.wait(&DRIVER::PULSE_LENGTH, do_modify){
            Ok(_) => self.current_direction = direction,
            Err(_) => return Err(()),
        }
        Ok(())
    }

    fn step(&mut self) -> Result<(), ()> {
        //////////////////////////////
        // Start step pulse
        match self.convert.wait(&DRIVER::PULSE_LENGTH, || {
            match self.driver.set_high() {
                Err(_) => return Err(()),
                Ok(_) => return Ok(()),
            };
        }) {
            Ok(_) => {
                return self.convert.wait(&DRIVER::PULSE_LENGTH, || {
                    match self.driver.set_low() {
                        Err(_) => return Err(()),
                        Ok(_) => return Ok(()),
                    };
                });
            }
            Err(_) => return Err(()),
        }
    }

    fn step_high(&mut self) {
        let _=self.driver.set_high();
    }
    fn step_low(&mut self) {
        let _=self.driver.set_low();
    }

    fn move_to_position(
        &mut self,
        target_accel: Num,
        max_velocity: Num,
        target_step: i32,
    ) -> Result<i32,i32> {
        let orig = self.current_step;
        let steps_from_here = target_step - self.current_step;

        let prof = StepProfile::new(target_accel);

        prof.0
            .borrow_mut()
            .enter_position_mode(max_velocity, steps_from_here.abs() as u32);

        let direction = if steps_from_here > 0 {
            Direction::Forward
        } else if steps_from_here < 0 {
            Direction::Backward
        } else {
            return Ok(0); // dont need move
        };
        if self.set_direction(direction).is_err() {
            return Err(0);
        }

        let mut stepped_num: usize = 0;
        while let Some(delay) = prof.0.borrow_mut().next_delay() {
            ///////////////////////////////////////
            match self.convert.wait(&DRIVER::PULSE_LENGTH, || {
                match self.driver.set_high() {
                    Err(_) => return Err(()),
                    Ok(_) => return Ok(()),
                };
            }) {
                Ok(_) => {
                    let delay = self.convert.rampdelay_to_nano(delay);
                    let delay_left = if delay < 2 * DRIVER::PULSE_LENGTH {
                        DRIVER::PULSE_LENGTH
                    } else {
                        delay - DRIVER::PULSE_LENGTH
                    };
                    let do_steplow = || match self.driver.set_low() {
                        Err(_) => return Err(()),
                        Ok(_) => return Ok(()),
                    };
                    match self.convert.wait(&delay_left, do_steplow) {
                        Ok(_) => {
                            self.current_step += self.current_direction as i32;
                            stepped_num += 1;
                        }
                        Err(_) => break,
                    }
                }
                Err(_) => break,
            }
        }

        if steps_from_here.abs() as usize == stepped_num {
            return Ok(self.current_step-orig);
        } else {
            return Err(self.current_step-orig);
        }

    }

    fn reset_position(&mut self, step: i32) -> Result<(), ()> {
        self.current_step = step;
        Ok(())
    }

    fn help_delay_ns(&mut self, timeout: u64) {
        let timeout = fugit::NanosDurationU64::from_ticks(timeout);
        let _ = self.convert.wait(&timeout, || Ok(()));
    }
}
//
