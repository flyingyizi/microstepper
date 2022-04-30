# How to Write a Driver

## min 

implement trait:  EnableDirectionControlTrait, EnableStepControlTrait, SetDirectionTrait, StepTrait,

if implement those traits, the MontionCtrl can be access by MotionControlTrait

## option

optinal implement trait: SetStepModeTrait, EnableStepModeControlTrait,
optinal implement trait: EnableResetControlTrait,ResetTrait,

if all of those implemented, the MontionCtrl can be access by MotionControlStepModeTrait.

## notes:

- your driver source can be locally, dont need included into this lib crate source. 

- from the driver's datasheet, we can know duration information, like driver's SETUP/HOLD/PLUSE duration. but when running in special hardware platform, maybe not running enough quickly, e.g. the mcu really wait 800ns when we set to waitting 200ns.in may sample, I set the PLUSE's high/low hold wdith to 2500ns, the wave freq ideally is 200KHz=(5000/1_000_000_000), but really hardwave output wave freq is 154.7Khz.

   so we should better check the really wave through Oscilloscope, then based on the really wave to modify like driver's SETUP/HOLD/PLUSE duration.


- 

## default driver

- [A4988](https://pdf1.alldatasheetcn.com/datasheet-pdf/view/338780/ALLEGRO/A4988.html)
- [DRV8825](https://www.ti.com/lit/ds/symlink/drv8825.pdf)
- [stspin220](https://html.alldatasheetcn.com/html-pdf/1246920/STMICROELECTRONICS/STSPIN220/31857/14/STSPIN220.html)
- soft: only provide PUL/DIR pin control, normal microstep driver hardware is like it. the SETUP/PLUSELENGTH should provided based the hardware's datasheet for example, 

    ```rust
    //! // [DMC432C](https://www.leisai.com/uploadfiles/2020/04/DM432C%E6%95%B0%E5%AD%97%E5%BC%8F%E4%B8%A4%E7%9B%B8%E6%AD%A5%E8%BF%9B%E9%A9%B1%E5%8A%A8%E5%99%A8%E4%BD%BF%E7%94%A8%E6%89%8B%E5%86%8C%20V1.10.pdf),
:
    //!   let driver = SOFT::<_,_,5000,2500>::new()
    //!       .enable_step_control(step_pin)
    //!       .enable_direction_control(dir_pin);
    ```
