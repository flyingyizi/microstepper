//! DRV8825 Driver
//!
// use core::convert::Infallible;

use embedded_hal::digital::v2::{OutputPin, PinState};

use crate::interfaces::{SetDirectionTrait, SetStepModeTrait, StepTrait,EnableDirectionControlTrait,
    EnableStepControlTrait, EnableStepModeControlTrait,
    EnableResetControlTrait,ResetTrait,
};



/// The DRV8825 driver API
///
pub struct DRV8825<Enable, Fault, Sleep, Reset, Mode0, Mode1, Mode2, Step, Dir>
{
    enable: Enable,
    fault: Fault,
    sleep: Sleep,
    reset: Reset,
    mode0: Mode0,
    mode1: Mode1,
    mode2: Mode2,
    step: Step,
    dir: Dir,
}

impl DRV8825<(), (), (), (), (), (), (), (), ()> {
    /// Create a new instance of `DRV8825`
    pub fn new() -> Self {
        Self {
            enable: (),
            fault: (),
            sleep: (),
            reset: (),
            mode0: (),
            mode1: (),
            mode2: (),
            step: (),
            dir: (),
        }
    }
}


impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError>
    EnableStepModeControlTrait<( Mode0, Mode1, Mode2)>
    for DRV8825<(), (), (), Reset, (), (), (), Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
    Mode0: OutputPin<Error = OutputPinError>,
    Mode1: OutputPin<Error = OutputPinError>,
    Mode2: OutputPin<Error = OutputPinError>,
{
    type WithStepModeControl =
        DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>;

    fn enable_step_mode_control(
        self,
        ( mode0, mode1, mode2): ( Mode0, Mode1, Mode2),
    ) -> Self::WithStepModeControl {
        DRV8825 {
            enable: self.enable,
            fault: self.fault,
            sleep: self.sleep,
            reset: self.reset,
            mode0,
            mode1,
            mode2,
            step: self.step,
            dir: self.dir,
        }
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError> SetStepModeTrait
    for DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
    Mode0: OutputPin<Error = OutputPinError>,
    Mode1: OutputPin<Error = OutputPinError>,
    Mode2: OutputPin<Error = OutputPinError>,
{
    // 7.6 Timing Requirements (page 7)
    // https://www.ti.com/lit/ds/symlink/drv8825.pdf
    const SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(650);
    const HOLD_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(650);

    type Error = OutputPinError;
    type StepMode = crate::step_mode::StepMode32;

    fn apply_mode_config(
        &mut self,
        step_mode: Self::StepMode,
    ) -> Result<(), Self::Error> {
        // Reset the device's internal logic and disable the h-bridge drivers.
        self.reset.set_low()?;

        use PinState::*;
        use crate::step_mode::StepMode32::*;
        let (mode0, mode1, mode2) = match step_mode {
            Full => (Low, Low, Low),
            M2 => (High, Low, Low),
            M4 => (Low, High, Low),
            M8 => (High, High, Low),
            M16 => (Low, Low, High),
            M32 => (High, High, High),
        };

        // Set mode signals.
        self.mode0.set_state(mode0)?;
        self.mode1.set_state(mode1)?;
        self.mode2.set_state(mode2)?;

        Ok(())
    }

}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError>
    EnableDirectionControlTrait<Dir>
    for DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, ()>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    type WithDirectionControl =
        DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>;

    fn enable_direction_control(self, dir: Dir) -> Self::WithDirectionControl {
        DRV8825 {
            enable: self.enable,
            fault: self.fault,
            sleep: self.sleep,
            reset: self.reset,
            mode0: self.mode0,
            mode1: self.mode1,
            mode2: self.mode2,
            step: self.step,
            dir,
        }
    }
}


impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError> SetDirectionTrait
    for DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>
where
    Dir: OutputPin<Error = OutputPinError>,
{
    // 7.6 Timing Requirements (page 7)
    // https://www.ti.com/lit/ds/symlink/drv8825.pdf
    const SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(650);

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

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError>
    EnableStepControlTrait<Step>
    for DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, (), Dir>
where
    Step: OutputPin<Error = OutputPinError>,
{
    type WithStepControl =
        DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>;

    fn enable_step_control(self, step: Step) -> Self::WithStepControl {
        DRV8825 {
            enable: self.enable,
            fault: self.fault,
            sleep: self.sleep,
            reset: self.reset,
            mode0: self.mode0,
            mode1: self.mode1,
            mode2: self.mode2,
            step,
            dir: self.dir,
        }
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError> StepTrait
    for DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>
where
    Step: OutputPin<Error = OutputPinError>,
{
    // 7.6 Timing Requirements (page 7)
    // https://www.ti.com/lit/ds/symlink/drv8825.pdf
    const PULSE_LENGTH: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(1900);

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
impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError>
    EnableResetControlTrait<Reset>
    for DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
{
    type WithResetControl =
        DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>;

    fn enable_reset_control(self, reset: Reset) -> Self::WithResetControl {
        DRV8825 {
            enable: self.enable,
            fault: self.fault,
            sleep: self.sleep,
            reset,
            mode0: self.mode0,
            mode1: self.mode1,
            mode2: self.mode2,
            step: self.step,
            dir: self.dir,
        }
    }
}

impl<Reset, Mode0, Mode1, Mode2, Step, Dir, OutputPinError> ResetTrait
    for DRV8825<(), (), (), Reset, Mode0, Mode1, Mode2, Step, Dir>
where
    Reset: OutputPin<Error = OutputPinError>,
{
    // 7.6 Timing Requirements (page 7)
    // https://www.ti.com/lit/ds/symlink/drv8825.pdf
    const RESET_SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(650);
    const RESET_HOLD_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(650);

    type Reset = Reset;
    type Error = OutputPinError;

    fn enable_driver(&mut self) -> Result<(), Self::Error> {
        self.reset.set_high()
    }

}
