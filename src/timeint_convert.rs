//
//
use super::DelayToTicksTrait;
use embedded_hal::timer::CountDown;
use embedded_time::{duration::Duration as _, rate::Fraction, ConversionError, TimeInt,duration::Generic};

/// TIMx should be a counter instance, the FREQ must be eq to the counter's FREQ
pub struct LpcHalCounterWrapper<TIMx, const FREQ: u32>(pub TIMx);

impl<TIMx, const FREQ: u32> DelayToTicksTrait for LpcHalCounterWrapper<TIMx, FREQ>
where
    TIMx: CountDown,
    <TIMx as CountDown>::Time: From<TimeInt>,
{
    fn wait(
        &mut self,
        timeout: &fugit::NanosDurationU64,
        mut closure: impl FnMut() -> Result<(), ()>,
    ) -> Result<(), ()> {
        unimplemnt!();
        // //convert to embedded_time Nanoseconds
        // let nano_ticks = timeout.ticks();
        // let duration = Generic::<u32>::new(nano_ticks, Fraction::new(1, 1000_000_000_u32));
        // let duration=embedded_time::duration::Nanoseconds::from(duration);
        
        // let ticks = duration.to_generic::<T>(Fraction::new(1, FREQ))?;

    }
}
