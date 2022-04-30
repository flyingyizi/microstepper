
## overview

it is customed project based on [stepper](https://crates.io/crates/stepper), recommend to use [`stepper`], it is a wonderful crate.

compare to the orig[`stepper`], modification include:

- restructure code as my will to simple Software hierarchy, e.g.  delete stepper's state FSA. replaced with a more directly way: each steper action is interact with driver, then responce to the app.

- to support "PUL/DIR/RESET" or "PUL/DIR" "PUL/DIR/RESET/MODx" hardware scenarioes, so I split orig motionctrl trait to montionctrl trait[`MotionControlTrait`] and MotionControlStepMode Trait[`MotionControlStepModeTrait`]. relatedly, split SetStepMode trait to SetStepMode trait[`SetStepModeTrait`] and Reset trait[`ResetTrait`] 

- to Support A4988 on hand and the microstep driver dm432c on hand

- add a normal micro-step driver[SOFT], it can special the PLUSE-LENGTH and STEUP length in app.

## framework

this stepper lib crate framework shows below.  the lib already defaultly provide stm32-hal convert,
and provide microstep driver for A 4988 and so on.

when you use this lib, you can write youself convert and microstep driver locally. only require you Follow trait(interface) requirements

```text
   ┌───────────────────────────────┐
   │   embedded app                │
   └───────────────────────────────┘
   ┌───────────────┐  ┌────────────┐
   │  MontionCtrl  │  │StepModeCtrl│
   └───────────────┘  └────────────┘
   ┌───────────────┐  ┌───────────────────┐
   │timer convert  │  │ microStep drivers │
   └───────────────┘  └───────────────────┘
   ┌───────────────────────────────┐
   │    embedded hardware          │
   └───────────────────────────────┘
```   

- convert: wrapper platform's timer counter. the counter fulfill `embedded_hal::timer::CountDown`. when you write your platform's convert, you need fulfill DelayToTicksTrait. the lib defaully provide convert for stm32-hal, see `[src/stm32f4xx_convert.rs]`

- drivers: refer `[src/drivers/how-to-write-a-driver.md]`

- MontionCtrl: refer `MotionControlTrait`, provide `move_to_position,set_direction,step,help_delay_ms` to app. in `move_to_position`, it internally use [ramp-maker](https://crates.io/crates/ramp-maker) to do Stepper Acceleration Ramp

- StepModeCtrl: refer `MotionControlStepModeTrait`, provide `set_step_mode`.


## usage example

```rust
//! stepper.
use stm32f4xx_hal as hal;
#[allow(unused_imports)]
use stm32f4xx_hal::{
    pac::{TIM4},
    timer::{CounterUs, Event},
};

// use hal::gpio::*;
#[allow(unused_imports)]
use microstepper::{
    step_mode::StepMode256, Direction, EnableDirectionControlTrait, EnableResetControlTrait,
    EnableStepControlTrait, EnableStepModeControlTrait, MontionCtrl, MotionControlStepModeTrait,
    MotionControlTrait, Num, STSPIN220,
};

//demo of init
type OutputPuhPull = hal::gpio::Output<hal::gpio::PushPull>;
pub fn init_motion(
    step_pin: hal::gpio::PA8<OutputPuhPull>,
    dir_pin: hal::gpio::PA9<OutputPuhPull>,
    convert: impl DelayToTicksTrait,
) -> impl MotionControlTrait {
    // driver is DM432CC, assign related pin to the driver
    let driver = SOFT::<_,_,5000,2500>::new()
        .enable_step_control(step_pin)
        .enable_direction_control(dir_pin);
    let mut ctrl = MontionCtrl::new(driver, convert);
    // set init state
    let _ = ctrl.set_direction(Direction::Forward);

    ctrl
}

```

main program sample:

```rust
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();
    ...
    let (step_pin, dir_pin) = (
        gpioa.pa8.into_push_pull_output(),
        gpioa.pa9.into_push_pull_output(),
    );
    ...
    // tim4 is 16-bit timer, use const FREQ to confirm wrapper know tim4's freq
    // attention, according PSC calc rule, coutner's freq must be less timer's 
    // input clock, and it must be divided by  timer's input clock with no remainder
    const FREQ: u32 = 50_000_000_u32;//50.MHz
    let counter = dp.TIM4.counter::<FREQ>(&clocks);
    // if the counter is 32-bit timer, here should fill with 32
    let convert = Stm32HalCounterWrapper::<_,16,FREQ>(counter); 
    let mut ctrl = stepperdemo::init_motion(step_pin, dir_pin, convert);

    // demo of how to use motion control
    fn at_will_demo(ctrl: &mut impl MotionControlTrait, times:u32) {
        let target_accel = Num::from_num(1000.123_f32); // steps per second^2
        let max_velocity = Num::from_num(23_u32); // steps per second

        // do circular(forward/backward) motion, each time move 1000 steps
        let mut steps=1000;
        for _ in 0..times {
            let _ = ctrl.move_to_position(max_velocity, target_accel, steps);
            steps = - steps;
        }
    }

```
![](https://github.com/flyingyizi/microstepper/blob/main/snap.PNG)

## about compat

some platform use fugit duration as the counter's Time, some platform use embedded-time duration as the couter's Time. so those work need to do based on platform.

topically, the STM32 hal use fugit duration, here provide Stm32HalCounterWrapper to wrapper , of course, you can provide yourself wrapper.

lpc8xx hal use embedded_time::TimeInt, orig [stepper](https://crates.io/crates/stepper) support it, because I not family with lpc8xx, so I not provide embedded_time::TimeInt wrapper now.

### example: stm32F4xx-hal implement convert

see src/stm32f4xx_convert.rs. 

usage sample see above section.


