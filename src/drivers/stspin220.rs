//! STSPIN220 Driver
//!
// use core::convert::Infallible;

use embedded_hal::digital::v2::{OutputPin, PinState};

use crate::interfaces::{
    EnableDirectionControlTrait, EnableResetControlTrait, EnableStepControlTrait,
    EnableStepModeControlTrait, ResetTrait, SetDirectionTrait, SetStepModeTrait, StepTrait,
};

/// The STSPIN220 driver API
///
pub struct STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4> {
    enable_fault: EnableFault,
    standby_reset: StandbyReset,
    mode1: Mode1,
    mode2: Mode2,
    step_mode3: StepMode3,
    dir_mode4: DirMode4,
}

impl STSPIN220<(), (), (), (), (), ()> {
    /// Create a new instance of `STSPIN220`
    pub fn new() -> Self {
        Self {
            enable_fault: (),
            standby_reset: (),
            mode1: (),
            mode2: (),
            step_mode3: (),
            dir_mode4: (),
        }
    }
}

impl<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4, OutputPinError>
    EnableStepModeControlTrait<( Mode1, Mode2)>
    for STSPIN220<EnableFault, StandbyReset, (), (), StepMode3, DirMode4>
where
    StandbyReset: OutputPin<Error = OutputPinError>,
    Mode1: OutputPin<Error = OutputPinError>,
    Mode2: OutputPin<Error = OutputPinError>,
    StepMode3: OutputPin<Error = OutputPinError>,
    DirMode4: OutputPin<Error = OutputPinError>,
{
    type WithStepModeControl =
        STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4>;

    fn enable_step_mode_control(
        self,
        ( mode1, mode2): ( Mode1, Mode2),
    ) -> Self::WithStepModeControl {
        STSPIN220 {
            enable_fault: self.enable_fault,
            standby_reset: self.standby_reset,
            mode1,
            mode2,
            step_mode3: self.step_mode3,
            dir_mode4: self.dir_mode4,
        }
    }
}

impl<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4, OutputPinError> SetStepModeTrait
    for STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4>
where
    StandbyReset: OutputPin<Error = OutputPinError>,
    Mode1: OutputPin<Error = OutputPinError>,
    Mode2: OutputPin<Error = OutputPinError>,
    StepMode3: OutputPin<Error = OutputPinError>,
    DirMode4: OutputPin<Error = OutputPinError>,
{
    const SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(1_000);
    const HOLD_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(100_000);

    type Error = OutputPinError;
    type StepMode = crate::step_mode::StepMode256;

    fn apply_mode_config(&mut self, step_mode: Self::StepMode) -> Result<(), Self::Error> {
        // Force driver into standby mode.
        self.standby_reset.set_low()?;

        use crate::step_mode::StepMode256::*;
        use PinState::*;
        let (mode1, mode2, mode3, mode4) = match step_mode {
            Full => (Low, Low, Low, Low),
            M2 => (High, Low, High, Low),
            M4 => (Low, High, Low, High),
            M8 => (High, High, High, Low),
            M16 => (High, High, High, High),
            M32 => (Low, High, Low, Low),
            M64 => (High, High, Low, High),
            M128 => (High, Low, Low, Low),
            M256 => (High, High, Low, Low),
        };

        // Set mode signals.
        self.mode1.set_state(mode1)?;
        self.mode2.set_state(mode2)?;
        self.step_mode3.set_state(mode3)?;
        self.dir_mode4.set_state(mode4)?;

        Ok(())
    }

}

impl<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4, OutputPinError>
    EnableDirectionControlTrait<DirMode4>
    for STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, ()>
where
    DirMode4: OutputPin<Error = OutputPinError>,
{
    type WithDirectionControl =
        STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4>;

    fn enable_direction_control(self, dir_mode4: DirMode4) -> Self::WithDirectionControl {
        STSPIN220 {
            enable_fault: self.enable_fault,
            standby_reset: self.standby_reset,
            mode1: self.mode1,
            mode2: self.mode2,
            step_mode3: self.step_mode3,
            dir_mode4,
        }
    }
}

impl<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4, OutputPinError> SetDirectionTrait
    for STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4>
where
    DirMode4: OutputPin<Error = OutputPinError>,
{
    const SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(100);

    type Dir = DirMode4;
    type Error = OutputPinError;

    fn set_backward(&mut self)-> Result<(), Self::Error>{
        self.dir_mode4.set_high()
    }
    fn set_forward(&mut self)-> Result<(), Self::Error>{
        self.dir_mode4.set_low()
    }
    #[inline(always)]
    fn dir_pin(&mut self) -> &mut Self::Dir{
        &mut self.dir_mode4
    }    
}

impl<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4, OutputPinError>
    EnableStepControlTrait<StepMode3>
    for STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, (), DirMode4>
where
    StepMode3: OutputPin<Error = OutputPinError>,
{
    type WithStepControl = STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4>;

    fn enable_step_control(self, step_mode3: StepMode3) -> Self::WithStepControl {
        STSPIN220 {
            enable_fault: self.enable_fault,
            standby_reset: self.standby_reset,
            mode1: self.mode1,
            mode2: self.mode2,
            step_mode3,
            dir_mode4: self.dir_mode4,
        }
    }
}

impl<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4, OutputPinError> StepTrait
    for STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4>
where
    StepMode3: OutputPin<Error = OutputPinError>,
{
    const PULSE_LENGTH: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(100);

    type Step = StepMode3;
    type Error = OutputPinError;

    fn set_high(&mut self)-> Result<(), Self::Error>{
        self.step_mode3.set_high()
    }
    fn set_low(&mut self)-> Result<(), Self::Error>{
        self.step_mode3.set_low()
    }
    fn setp_pin(&mut self) -> &mut Self::Step{
        &mut self.step_mode3
    }    

}

//////////////////////////////////////////////////
impl<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4, OutputPinError>
    EnableResetControlTrait<StandbyReset>
    for STSPIN220<EnableFault, (), Mode1, Mode2, StepMode3, DirMode4>
where
    StandbyReset: OutputPin<Error = OutputPinError>,
{
    type WithResetControl = STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4>;

    fn enable_reset_control(self, standby_reset: StandbyReset) -> Self::WithResetControl {
        STSPIN220 {
            enable_fault: self.enable_fault,
            standby_reset,
            mode1: self.mode1,
            mode2: self.mode2,
            step_mode3: self.step_mode3,
            dir_mode4: self.dir_mode4,
        }
    }
}

impl<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4, OutputPinError> ResetTrait
    for STSPIN220<EnableFault, StandbyReset, Mode1, Mode2, StepMode3, DirMode4>
where
    StandbyReset: OutputPin<Error = OutputPinError>,
{
    const RESET_SETUP_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(1_000);
    const RESET_HOLD_TIME: fugit::NanosDurationU64 = fugit::NanosDurationU64::from_ticks(100_000);

    type Reset = StandbyReset;
    type Error = OutputPinError;

    fn enable_driver(&mut self) -> Result<(), Self::Error> {
        // Leave standby mode.
        self.standby_reset.set_high()
    }

}
