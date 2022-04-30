///! according to STM32F4XX-HAL source(tm32f4xx-hal\src\timer\hal_02.rs), the counter's
///! CountDown:Time is TimerDurationU32<FREQ>
//
//
use super::DelayToTicksTrait;
use embedded_hal::timer::CountDown;

/// stm32 counter wrapper implement [DelayToTicksTrait]
/// TIMx should be a counter instance, the FREQ must be eq to the counter's FREQ
/// the LEN must be eq to the counter's bit width.
///
/// e.g. for stm32F4, TIM4 is 16-bit timer, the TIM2 is 32-bit timer
///  // remember the counter's FREQ should be less counter;s clock source
///  // and fulfill "assert!(clock_source % FREQ == 0)"
///  const FREQ: u32 = 1_000_000_u32;
///  let mut counter = dp.TIM4.counter::<FREQ>(&clocks);
///  // tim4 is a 16 bit counter, so code 16
///  let convert = Stm32HalCounterWrapper::<_,16,FREQ>(counter);

pub struct Stm32HalCounterWrapper<TIMx, const LEN: u32, const FREQ: u32>(pub TIMx);

impl<TIMx, const LEN: u32, const FREQ: u32> DelayToTicksTrait
    for Stm32HalCounterWrapper<TIMx, LEN, FREQ>
where
    TIMx: CountDown,
    <TIMx as CountDown>::Time: From<fugit::TimerDurationU32<FREQ>>,
{
    fn wait(
        &mut self,
        timeout: &fugit::NanosDurationU64,
        mut closure: impl FnMut() -> Result<(), ()>,
    ) -> Result<(), ()> {
        let mut ignore_timeout: bool = false;
        // convert to counter's base
        let timeout: Option<fugit::TimerDurationU64<FREQ>> = timeout.const_try_into();
        if timeout.is_none() {
            ignore_timeout = true;
        }
        let timeout = timeout.unwrap();

        let max: u32 = (1 << LEN) - 1;
        let total_ticks = timeout.ticks();
        if total_ticks < 1 {
            //we think this scenario means dont need wait timeout, maybe you
            // can provide yourself wrapper use like "cortex_m::asm::nop()" to do a
            // approximate timeout
            ignore_timeout = true;
        }

        if ignore_timeout == false {
            let rem = total_ticks % (max as u64);
            let remainder: fugit::TimerDurationU32<FREQ> =
                fugit::TimerDurationU32::<FREQ>::from_ticks(rem as u32);
            self.0.start(remainder);
        }
        // Invoke closure
        if let Err(_) = closure() {
            return Err(());
        }

        if ignore_timeout == false {
            match nb::block!(self.0.wait()) {
                Ok(_) => {}
                Err(_) => return Err(()),
            }

            // maybe upper counter's top limit
            if total_ticks > max as u64 {
                let one: fugit::TimerDurationU32<FREQ> =
                    fugit::TimerDurationU32::<FREQ>::from_ticks(max);

                let n = total_ticks / (max as u64);

                for _i in 0..n {
                    self.0.start(one.clone());
                    match nb::block!(self.0.wait()) {
                        Ok(_) => {}
                        Err(_) => return Err(()),
                    }
                }
            }
        }

        Ok(())
    }
}

// impl<TIMx, const FREQ: u32> CountDown for Stm32HalCounterWrapper<TIMx, FREQ>
// where
//     TIMx: embedded_hal::CountDown,
// {
//     type Time = TimerDurationU32<FREQ>;

//     fn start<T>(&mut self, timeout: T)
//     where
//         T: Into<Self::Time>,
//     {
//         self.0.start(timeout.into()).unwrap()
//     }

//     fn wait(&mut self) -> nb::Result<(), Void> {
//         return self.0.wait();
//     }
// }
