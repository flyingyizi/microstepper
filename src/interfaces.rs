//! Traits that can be implemented by Stepper drivers
//!

use embedded_hal::digital::v2::OutputPin;
use super::Direction;
pub type Num = ramp_maker::trapezoidal::DefaultNum;
// use core::convert::Infallible;

/// Enable microstepping mode control for a driver
///
/// The `Resources` type parameter defines the hardware resources required for
/// controlling microstepping mode.
pub trait EnableStepModeControlTrait<Resources> {
    /// The type of the driver after microstepping mode control has been enabled
    type WithStepModeControl: SetStepModeTrait;

    /// Enable microstepping mode control
    fn enable_step_mode_control(
        self,
        res: Resources,
    ) -> Self::WithStepModeControl;
}

/// Implemented by drivers that support controlling the microstepping mode
pub trait SetStepModeTrait {
    /// The time the mode signals need to be held before re-enabling the driver
    const SETUP_TIME: fugit::NanosDurationU64;

    /// The time the mode signals need to be held after re-enabling the driver
    const HOLD_TIME: fugit::NanosDurationU64;

    /// The error that can occur while using this trait
    type Error;
    /// The type that defines the microstepping mode
    type StepMode;

    ///
    /// Apply the new step mode configuration
    ///
    /// Typically this puts the driver into reset and sets the mode pins
    /// according to the new step mode. natation: additonally Re-enable the driver after the mode has been set
    fn apply_mode_config(
        &mut self,
        step_mode: Self::StepMode,
    ) -> Result<(), Self::Error>;

}

/// Enable direction control for a driver
///
/// The `Resources` type parameter defines the hardware resources required for
/// direction control.
pub trait EnableDirectionControlTrait<Resources> {
    /// The type of the driver after direction control has been enabled
    type WithDirectionControl: SetDirectionTrait;

    /// Enable direction control
    fn enable_direction_control(
        self,
        res: Resources,
    ) -> Self::WithDirectionControl;
}

/// Implemented by drivers that support controlling the DIR signal
pub trait SetDirectionTrait {
    /// The time that the DIR signal must be held for a change to apply
    const SETUP_TIME: fugit::NanosDurationU64;

    /// The type of the DIR pin
    type Dir: OutputPin;

    /// The error that can occur while accessing the DIR pin
    type Error;

    fn set_backward(&mut self)-> Result<(), Self::Error>;
    fn set_forward(&mut self)-> Result<(), Self::Error>;
    fn dir_pin(&mut self) -> &mut Self::Dir;
}

/// Enable step control for a driver
///
/// The `Resources` type parameter defines the hardware resources required for
/// step control.
pub trait EnableStepControlTrait<Resources> {
    /// The type of the driver after step control has been enabled
    type WithStepControl;

    /// Enable step control
    fn enable_step_control(self, res: Resources) -> Self::WithStepControl;
}

/// Implemented by drivers that support controlling the STEP signal
pub trait StepTrait {
    /// The minimum length of a STEP pulse
    const PULSE_LENGTH: fugit::NanosDurationU64;

    /// The type of the STEP pin
    type Step: OutputPin;

    /// The error that can occur while accessing the STEP pin
    type Error;

    /// a step include high + low
    fn set_high(&mut self)-> Result<(), Self::Error>;
    fn set_low(&mut self)-> Result<(), Self::Error>;
    fn setp_pin(&mut self) -> &mut Self::Step;

}


/// Implemented by drivers that have motion control capabilities
///
pub trait MotionControlTrait {
    /// Move to the given position. accel unit is steps per second^2, velocity unit is steps per second
    /// result repcent completed steps.
    /// the target_step is a value from home pos. for example, priviously has arrive pos(target_step=Y),
    /// when you call it again with target_step=Y, the motion will not move. if you call it with target_step=Y-1,
    /// the motion will move backward one step
    /// 
    ///       negative                          positive
    ///    <------------------------------+--------------------------->
    ///         backward                home      forward
    fn move_to_position(
        &mut self,
        target_accel: Num,
        max_velocity: Num,
        target_step: i32,
    ) -> Result<i32,i32>;

    /// Reset internal position to the given value
    ///
    /// This method must not start a motion. Its only purpose is to change the
    /// driver's internal position value, for example for homing.
    fn reset_position(&mut self, step: i32) -> Result<(), ()>;

    fn set_direction(&mut self, direction: Direction) -> Result<(), ()>;
    fn step(&mut self) -> Result<(), ()>;
    fn step_high(&mut self);
    fn step_low(&mut self);
    
    //helper, use montionctrl's counter do some delays
    fn help_delay_ns(&mut self, timeout: u64);
}

/// Implemented by drivers that have motion control capabilities
///
pub trait MotionControlStepModeTrait {
    type  StepMode;
    /// Set step mode of the wrapped driver
    ///
    fn set_step_mode(&mut self, step_mode: Self::StepMode) -> Result<(), ()>;
}


/// Converts delay values from RampMaker into timer ticks
///
/// RampMaker is agnostic over the units used, and the unit of the timer ticks
/// depend on the target platform. This trait allows Stepper to convert between
/// both types. The user must supply an implementation that matches their
/// environment.
///
/// the lib crate default implement for stm32-hal. refer[Stm32HalCounterWrapper]
pub trait DelayToTicksTrait {
    /// Convert ramp delay value into fugit::NanosDurationU64
    fn rampdelay_to_nano(&self, delay: Num) -> fugit::NanosDurationU64 {
        let delay_s = delay;
        // let ticks = mrt::Ticks::try_from(Seconds(delay_s.int().to_num())).unwrap();
        let mut out = fugit::NanosDurationU64::secs( delay_s.int().to_num() );

        let delay_ms = delay_s.frac() * 1000;
        out += fugit::NanosDurationU64::millis( delay_ms.int().to_num() );

        let delay_us = delay_ms.frac() * 1000;
        out += fugit::NanosDurationU64::micros( delay_us.int().to_num() );       

        let delay_ns = delay_us.frac() * 1000;
        out +=fugit::NanosDurationU64::nanos( delay_ns.int().to_num() );

        out
    }

    /// platform implement,
    /// internal logic is: start counter---> do closure action ---> wait until timeout
    /// if return err, will cause skip next actions in the flow that called it.
    /// so you should better carefully deal it.
    fn wait(&mut self, timeout: &fugit::NanosDurationU64, closure: impl FnMut()-> Result<(), ()>) -> Result<(), ()>;
    
}



//////////////////////////////
pub trait EnableResetControlTrait<Resources> {
    /// The type of the driver after microstepping mode control has been enabled
    type WithResetControl: ResetTrait;

    /// Enable reset control
    fn enable_reset_control(self, res: Resources) -> Self::WithResetControl;
}

pub trait ResetTrait {
    /// The time the mode signals need to be held before re-enabling the driver
    const RESET_SETUP_TIME: fugit::NanosDurationU64;

    /// The time the mode signals need to be held after re-enabling the driver
    const RESET_HOLD_TIME: fugit::NanosDurationU64;

    /// The type of the STEP pin
    type Reset: OutputPin;

    /// The error that can occur while accessing the STEP pin
    type Error;
        /// 
    fn enable_driver(&mut self) -> Result<(), Self::Error>;

}


