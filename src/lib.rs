// #![no_std]
#![cfg_attr(not(test), no_std)]

mod drivers;
mod interfaces;
mod main;
// pub mod compat;
// pub mod compat_fugit;
pub mod step_mode;
pub mod stm32f4xx_convert;
pub use drivers::{drv8825::DRV8825, stspin220::STSPIN220,a4988::A4988,soft::SOFT};
pub use interfaces::{
    EnableDirectionControlTrait, EnableStepControlTrait,EnableResetControlTrait,
    EnableStepModeControlTrait, SetDirectionTrait, SetStepModeTrait, StepTrait,ResetTrait,
    Num,MotionControlTrait,MotionControlStepModeTrait,DelayToTicksTrait,
};
pub use main::MontionCtrl;

pub extern crate embedded_hal;
pub extern crate fixed;
pub extern crate ramp_maker;
pub extern crate fugit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Rotate the motor forward
    ///
    /// This corresponds to whatever direction the motor rotates in when the
    /// driver's DIR signal is set HIGH.
    Forward = 1,

    /// Rotate the motor backward
    ///
    /// This corresponds to whatever direction the motor rotates in when the
    /// driver's DIR signal set is LOW.
    Backward = -1,
}

// use core::convert::{/*Infallible,*/ TryFrom, TryInto as _};
/// only timer's internal err return error
//



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


}
