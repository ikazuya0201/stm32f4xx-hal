use core::marker::PhantomData;
use core::mem;

use crate::hal;
use crate::stm32::{RCC, TIM1};
use cast::{u16, u32};

use crate::gpio::gpioa::{PA10, PA11, PA8, PA9};
use crate::gpio::{Alternate, AF1};
use crate::time::Hertz;
use crate::timer::Timer;

pub trait Pins<TIM> {
    const C1: bool;
    const C2: bool;
    const C3: bool;
    const C4: bool;
    type Channels;
}

impl Pins<TIM1>
    for (
        PA8<Alternate<AF1>>,
        PA9<Alternate<AF1>>,
        PA10<Alternate<AF1>>,
        PA11<Alternate<AF1>>,
    )
{
    const C1: bool = true;
    const C2: bool = true;
    const C3: bool = true;
    const C4: bool = true;
    type Channels = (Pwm<TIM1, C1>, Pwm<TIM1, C2>, Pwm<TIM1, C3>, Pwm<TIM1, C4>);
}

impl Timer<TIM1> {
    pub fn pwm<PINS, T>(self, _pins: PINS, freq: T) -> PINS::Channels
    where
        PINS: Pins<TIM1>,
        T: Into<Hertz>,
    {
        let Self { clocks, tim } = self;
        tim1(tim, _pins, freq.into(), clocks.pclk2())
    }
}

pub struct Pwm<TIM, CHANNEL> {
    _channel: PhantomData<CHANNEL>,
    _tim: PhantomData<TIM>,
}

pub struct C1;
pub struct C2;
pub struct C3;
pub struct C4;

macro_rules! hal_advanced {
    ($($TIMX: ident: ($timX: ident),)+) => {
        $(
            fn $timX<PINS> (
                tim: $TIMX,
                _pins: PINS,
                freq: Hertz,
                clk: Hertz,
            ) -> PINS::Channels
            where PINS: Pins<$TIMX>,
            {
                let rcc = unsafe { &(*RCC::ptr()) };
                rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

                tim.cr1.modify(|_, w| w.cen().clear_bit());

                if PINS::C1 {
                    tim.ccmr1_output().modify(|_, w| w.oc1pe().set_bit().oc1m().pwm_mode1());
                }

                if PINS::C2 {
                    tim.ccmr1_output().modify(|_, w| w.oc2pe().set_bit().oc2m().pwm_mode1());
                }

                if PINS::C3 {
                    tim.ccmr2_output().modify(|_, w| w.oc3pe().set_bit().oc3m().pwm_mode1());
                }

                if PINS::C4 {
                    tim.ccmr2_output().modify(|_, w| w.oc4pe().set_bit().oc4m().pwm_mode1());
                }

                let ticks = clk.0 / freq.0;
                let psc = u16(ticks / (1<<16)).unwrap();
                tim.psc.write(|w| w.psc().bits(psc));
                let arr = u16(ticks / u32(psc+1)).unwrap() - 1;
                tim.arr.write(|w| w.arr().bits(arr));

                tim.egr.write(|w| w.ug().set_bit());
                tim.cr1.modify(|_, w| w.arpe().set_bit());

                tim.bdtr.modify(|_, w| w.ossr().clear_bit());
                tim.bdtr.modify(|_, w| w.moe().set_bit());

                tim.cr1.modify(|_, w| w.cen().set_bit());

                unsafe{ mem::uninitialized() }
            }

            impl hal::PwmPin for Pwm<$TIMX, C1> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe{ &(*$TIMX::ptr())}.ccer.modify(|_,w| w.cc1e().clear_bit());
                }

                fn enable(&mut self) {
                    unsafe {&(*$TIMX::ptr())}.ccer.modify(|_,w| w.cc1e().set_bit().cc1ne().clear_bit());
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe {&(*$TIMX::ptr())}.ccr1.read().ccr().bits()
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe {&(*$TIMX::ptr())}.arr.read().arr().bits()
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                    unsafe {&(*$TIMX::ptr()).ccr2.write(|w| w.ccr().bits(duty))};
                }
            }

            impl hal::PwmPin for Pwm<$TIMX, C2> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe{ &(*$TIMX::ptr())}.ccer.modify(|_,w| w.cc2e().clear_bit());
                }

                fn enable(&mut self) {
                    unsafe {&(*$TIMX::ptr())}.ccer.modify(|_,w| w.cc2e().set_bit().cc2ne().clear_bit());
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe {&(*$TIMX::ptr())}.ccr2.read().ccr().bits()
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe {&(*$TIMX::ptr())}.arr.read().arr().bits()
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                    unsafe {&(*$TIMX::ptr()).ccr2.write(|w| w.ccr().bits(duty))};
                }
            }

            impl hal::PwmPin for Pwm<$TIMX, C3> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe{ &(*$TIMX::ptr())}.ccer.modify(|_,w| w.cc3e().clear_bit());
                }

                fn enable(&mut self) {
                    unsafe {&(*$TIMX::ptr())}.ccer.modify(|_,w| w.cc3e().set_bit().cc3ne().clear_bit());
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe {&(*$TIMX::ptr())}.ccr3.read().ccr().bits()
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe {&(*$TIMX::ptr())}.arr.read().arr().bits()
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                    unsafe {&(*$TIMX::ptr()).ccr3.write(|w| w.ccr().bits(duty))};
                }
            }

            impl hal::PwmPin for Pwm<$TIMX, C4> {
                type Duty = u16;

                fn disable(&mut self) {
                    unsafe{ &(*$TIMX::ptr())}.ccer.modify(|_, w| w.cc4e().clear_bit());
                }

                fn enable(&mut self) {
                    unsafe {&(*$TIMX::ptr())}.ccer.modify(|_, w| w.cc4e().set_bit());
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe {&(*$TIMX::ptr())}.ccr4.read().ccr().bits()
                }

                fn get_max_duty(&self) -> Self::Duty {
                    unsafe {&(*$TIMX::ptr())}.arr.read().arr().bits()
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                    unsafe {&(*$TIMX::ptr()).ccr4.write(|w| w.ccr().bits(duty))};
                }
            }
        )+
    }
}

hal_advanced! {
    TIM1: (tim1),
}
