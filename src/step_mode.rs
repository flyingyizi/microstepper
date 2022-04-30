// use core::convert::TryFrom;
// use core::prelude::*;
// use core::result::*;
/// Indicates that a given step mode value did not represent a valid step mode
///
/// Returned by the `TryFrom` implementations of the various step mode enums.

#[doc = "Defines the microstepping mode for drivers with a resolution 
    of up to 256 microsteps"]
#[derive(Clone,Debug,Copy, /**/  Eq, PartialEq, Ord, PartialOrd)]
pub enum StepMode256 {
    Full=1,
    M2=2,
    M4=4,
    M8=8,
    M16=16,
    M32=32,
    M64=64,
    M128=128,
    M256=256,
}

#[doc = "Defines the microstepping mode for drivers with a resolution 
    of up to 32 microsteps"]
#[derive(Clone,Debug,Copy, /**/  Eq, PartialEq, Ord, PartialOrd)]
pub enum StepMode32 {
    Full=1,
    M2=2,
    M4=4,
    M8=8,
    M16=16,
    M32=32,
}
#[doc = "Defines the microstepping mode for drivers with a resolution 
    of up to 16 microsteps"]
#[derive(Clone,Debug,Copy, /**/  Eq, PartialEq, Ord, PartialOrd)]
pub enum StepMode16 {
    Full=1,
    M2=2,
    M4=4,
    M8=8,
    M16=16,
}
pub enum StepMode1 {
    Full=1,
}
