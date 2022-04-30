// use crate::interfaces::DelayToTicksTrait;
use ramp_maker::{/*MotionProfile,*/Trapezoidal};
use core::cell::RefCell;
pub type Num = crate::interfaces::Num;

// FREQ should be equal to counter freq that used byMontionCtrl
pub struct StepProfile(pub RefCell<Trapezoidal>);

impl StepProfile {
    pub fn new(target_accel: Num) -> Self {
        Self( RefCell::new(Trapezoidal::new(target_accel)))
    }
}

// // for counter_ns ,
