//! soft  Driver
//!
//! only support PUL/DIR
// DM432C:
//!   let driver = SOFT::<_,_,5000,2500>::new()
//!       .enable_step_control(step_pin)
//!       .enable_direction_control(dir_pin);

use embedded_hal::digital::v2::OutputPin;

use crate::interfaces::{
    EnableDirectionControlTrait, EnableStepControlTrait, SetDirectionTrait, StepTrait,
};

/// The SOFT driver API
/// STEUP: DIR setup time, unit is ns
/// PULLEN: PULSE length, unit is ns
pub struct SOFT<Step, Dir, const STEUP: u32, const PULLEN: u32> {
    step: Step,
    dir: Dir,
}

impl<const STEUP: u32, const PULLEN: u32> SOFT<(), (), STEUP, PULLEN> {
    /// Create a new instance of `SOFT`
    pub fn new() -> Self {
        Self { step: (), dir: () }
    }
}

impl<Step, Dir, OutputPinError, const STEUP: u32, const PULLEN: u32>
    EnableDirectionControlTrait<Dir> for SOFT<Step, (), STEUP, PULLEN>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    type WithDirectionControl = SOFT<Step, Dir, STEUP, PULLEN>;

    fn enable_direction_control(self, dir: Dir) -> Self::WithDirectionControl {
        SOFT {
            step: self.step,
            dir,
        }
    }
}

impl<Step, Dir, OutputPinError, const STEUP: u32, const PULLEN: u32> SetDirectionTrait
    for SOFT<Step, Dir, STEUP, PULLEN>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    // const SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(5000);
    const SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(STEUP as u64);

    type Dir = Dir;
    type Error = OutputPinError;

    fn set_backward(&mut self) -> Result<(), Self::Error> {
        self.dir.set_high()
    }
    fn set_forward(&mut self) -> Result<(), Self::Error> {
        self.dir.set_low()
    }
    #[inline(always)]
    fn dir_pin(&mut self) -> &mut Self::Dir{
        &mut self.dir
    }    
}

impl<Step, Dir, OutputPinError, const STEUP: u32, const PULLEN: u32> EnableStepControlTrait<Step>
    for SOFT<(), Dir, STEUP, PULLEN>
where
    Step: OutputPin<Error = OutputPinError>,
{
    type WithStepControl = SOFT<Step, Dir, STEUP, PULLEN>;

    fn enable_step_control(self, step: Step) -> Self::WithStepControl {
        SOFT {
            step,
            dir: self.dir,
        }
    }
}

impl<Step, Dir, OutputPinError, const STEUP: u32, const PULLEN: u32> StepTrait
    for SOFT<Step, Dir, STEUP, PULLEN>
where
    Step: OutputPin<Error = OutputPinError>,
{
    // const PULSE_LENGTH: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(2500);
    const PULSE_LENGTH: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(PULLEN as u64);

    type Step = Step;
    type Error = OutputPinError;

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.step.set_high()
    }
    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.step.set_low()
    }

    #[inline(always)]
    fn setp_pin(&mut self) -> &mut Self::Step{
        &mut self.step
    }    
}
