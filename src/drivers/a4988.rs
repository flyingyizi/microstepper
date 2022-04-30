//! A4988 Driver
//!
//! Platform-agnostic driver API for the A4988 stepper motor driver. Can be
//! used on any platform for which implementations of the required
//! [embedded-hal] traits are available.
//! [embedded-hal]: https://crates.io/crates/embedded-hal

// use core::convert::Infallible;

use embedded_hal::digital::v2::{OutputPin, PinState};

use crate::interfaces::{SetDirectionTrait, SetStepModeTrait, StepTrait,EnableDirectionControlTrait,
        EnableStepControlTrait, EnableStepModeControlTrait,
        EnableResetControlTrait,ResetTrait,
};


/// The A4988 driver API
///
pub struct A4988<Enable, Sleep, Reset, Ms1, Ms2, Ms3, Step, Dir>
{
    enable: Enable,
    sleep: Sleep,

    reset: Reset,
    ms1: Ms1,
    ms2: Ms2,
    ms3: Ms3,
    step: Step,
    dir: Dir,
}

impl A4988<(), (), (), (), (), (), (), ()> {
    /// Create a new instance of `A4988`
    pub fn new() -> Self {
        Self {
            enable: (),
            sleep: (),
            reset: (),
            ms1: (),
            ms2: (),
            ms3: (),
            step: (),
            dir: (),
        }
    }
}


impl<Reset, Ms1, Ms2, Ms3, Step, Dir, OutputPinError>
    EnableStepModeControlTrait<( Ms1, Ms2, Ms3)>
    for A4988<(), (),Reset, (),  (), (), Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
    Ms3: OutputPin<Error = OutputPinError>,
    Ms1: OutputPin<Error = OutputPinError>,
    Ms2: OutputPin<Error = OutputPinError>,
{
    type WithStepModeControl =
        A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, Dir>;

    fn enable_step_mode_control(
        self,
        ( ms1, ms2, ms3): ( Ms1, Ms2, Ms3),
    ) -> Self::WithStepModeControl {
        A4988 {
            enable: self.enable,
            sleep: self.sleep,
            reset: self.reset,
            ms1,
            ms2,
            ms3,
            step: self.step,
            dir: self.dir,
        }
    }
}

impl<Reset, Ms1, Ms2, Ms3, Step, Dir, OutputPinError> SetStepModeTrait
    for A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, Dir>

    where
    Reset: OutputPin<Error = OutputPinError>,
    Ms3: OutputPin<Error = OutputPinError>,
    Ms1: OutputPin<Error = OutputPinError>,
    Ms2: OutputPin<Error = OutputPinError>,
{
    // 7.6 Timing Requirements (page 7)
    const SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(200);
    const HOLD_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(200);

    type Error = OutputPinError;
    type StepMode = crate::step_mode::StepMode16;

    fn apply_mode_config(
        &mut self,
        step_mode: Self::StepMode,
    ) -> Result<(), Self::Error> {
        //P7, Reset the device's internal logic and disable the h-bridge drivers.
        self.reset.set_low()?;

        use PinState::*;
        use crate::step_mode::StepMode16::*;
        let ( ms1, ms2,ms3) = match step_mode {
            Full => (Low, Low, Low),
            M2 => (High, Low, Low),
            M4 => (Low, High, Low),
            M8 => (High, High, Low),
            M16 =>(High, High, High),
        };

        // Set mode signals.
        self.ms1.set_state(ms1)?;
        self.ms2.set_state(ms2)?;
        self.ms3.set_state(ms3)?;

        Ok(())
    }

}

impl<Reset, Ms1, Ms2, Ms3, Step, Dir, OutputPinError>
    EnableDirectionControlTrait<Dir>
    for A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, ()>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    type WithDirectionControl =
        A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, Dir>;

    fn enable_direction_control(self, dir: Dir) -> Self::WithDirectionControl {
        A4988 {
            enable: self.enable,
            sleep: self.sleep,
            reset: self.reset,
            ms3: self.ms3,
            ms1: self.ms1,
            ms2: self.ms2,
            step: self.step,
            dir,
        }
    }
}


impl<Reset, Ms1, Ms2, Ms3, Step, Dir, OutputPinError> SetDirectionTrait
    for A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, Dir>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    // P6
    const SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(200);

    type Dir = Dir;
    type Error = OutputPinError;

    fn set_backward(&mut self)-> Result<(), Self::Error>{
        self.dir.set_high()
    }
    fn set_forward(&mut self)-> Result<(), Self::Error>{
        self.dir.set_low()
    }
    #[inline(always)]
    fn dir_pin(&mut self) -> &mut Self::Dir{
        &mut self.dir
    }    
}

impl<Reset, Ms1, Ms2, Ms3, Step, Dir, OutputPinError>
    EnableStepControlTrait<Step>
    for A4988< (), (), Reset, Ms1, Ms2, Ms3, (), Dir>
where
    Step: OutputPin<Error = OutputPinError>,
{
    type WithStepControl =
        A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, Dir>;

    fn enable_step_control(self, step: Step) -> Self::WithStepControl {
        A4988 {
            enable: self.enable,
            sleep: self.sleep,
            reset: self.reset,
            ms3: self.ms3,
            ms1: self.ms1,
            ms2: self.ms2,
            step,
            dir: self.dir,
        }
    }
}

impl<Reset, Ms1, Ms2, Ms3, Step, Dir, OutputPinError> StepTrait
    for A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, Dir>
where
    Step: OutputPin<Error = OutputPinError>,
{
    // P6
    const PULSE_LENGTH: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(1000);

    type Step = Step;
    type Error = OutputPinError;

    fn set_high(&mut self)-> Result<(), Self::Error>{
        self.step.set_high()
    }
    fn set_low(&mut self)-> Result<(), Self::Error>{
        self.step.set_low()
    }
    fn setp_pin(&mut self) -> &mut Self::Step{
        &mut self.step
    }    

}

//////////////////////////////////////////////////
impl<Reset, Ms1, Ms2, Ms3, Step, Dir, OutputPinError>
    EnableResetControlTrait<Reset>
    for A4988< (), (), (), Ms1, Ms2, Ms3, Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
{
    type WithResetControl =
        A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, Dir>;

    fn enable_reset_control(self, reset: Reset) -> Self::WithResetControl {
        A4988 {
            enable: self.enable,
            sleep: self.sleep,
            reset,
            ms3: self.ms3,
            ms1: self.ms1,
            ms2: self.ms2,
            step:self.step,
            dir: self.dir,
        }
    }
}

impl<Reset, Ms1, Ms2, Ms3, Step, Dir, OutputPinError> ResetTrait
    for A4988< (), (), Reset, Ms1, Ms2, Ms3, Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
{
    const RESET_SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(200);
    const RESET_HOLD_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(200);

    type Reset = Reset;
    type Error = OutputPinError;

    fn enable_driver(&mut self) -> Result<(), Self::Error> {
        self.reset.set_high()
    }

}
