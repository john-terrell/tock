//! Interrupt mapping and DMA channel setup.

use cortexm4;
use crccu;
use gpio;
use kernel::Chip;
use nvic;

pub struct Stm32F407 {
    mpu: cortexm4::mpu::MPU,
    userspace_kernel_boundary: cortexm4::syscall::SysCall,
    systick: cortexm4::systick::SysTick,
}

impl Stm32F407 {
    pub unsafe fn new() -> Stm32F407 {
        Stm32F407 {
            mpu: cortexm4::mpu::MPU::new(),
            userspace_kernel_boundary: cortexm4::syscall::SysCall::new(),
            systick: cortexm4::systick::SysTick::new(),
        }
    }
}

impl Chip for Stm32F407 {
    type MPU = cortexm4::mpu::MPU;
    type UserspaceKernelBoundary = cortexm4::syscall::SysCall;
    type SysTick = cortexm4::systick::SysTick;

    fn service_pending_interrupts(&self) {
        unsafe {
            loop {
                if let Some(interrupt) = cortexm4::nvic::next_pending() {
                    match interrupt {
                        nvic::ASTALARM => ast::AST.handle_interrupt(),

                        nvic::GPIO0 => gpio::PA.handle_interrupt(),
                        nvic::GPIO1 => gpio::PA.handle_interrupt(),
                        nvic::GPIO2 => gpio::PA.handle_interrupt(),
                        nvic::GPIO3 => gpio::PA.handle_interrupt(),
                        nvic::GPIO4 => gpio::PB.handle_interrupt(),
                        nvic::GPIO5 => gpio::PB.handle_interrupt(),
                        nvic::GPIO6 => gpio::PB.handle_interrupt(),
                        nvic::GPIO7 => gpio::PB.handle_interrupt(),
                        nvic::GPIO8 => gpio::PC.handle_interrupt(),
                        nvic::GPIO9 => gpio::PC.handle_interrupt(),
                        nvic::GPIO10 => gpio::PC.handle_interrupt(),
                        nvic::GPIO11 => gpio::PC.handle_interrupt(),

                        _ => {
                            panic!("unhandled interrupt {}", interrupt);
                        }
                    }
                    let n = cortexm4::nvic::Nvic::new(interrupt);
                    n.clear_pending();
                    n.enable();
                } else {
                    break;
                }
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { cortexm4::nvic::has_pending() || deferred_call::has_tasks() }
    }

    fn mpu(&self) -> &cortexm4::mpu::MPU {
        &self.mpu
    }

    fn systick(&self) -> &cortexm4::systick::SysTick {
        &self.systick
    }

    fn userspace_kernel_boundary(&self) -> &cortexm4::syscall::SysCall {
        &self.userspace_kernel_boundary
    }

    fn sleep(&self) {
        if pm::deep_sleep_ready() {
            unsafe {
                cortexm4::scb::set_sleepdeep();
            }
        } else {
            unsafe {
                cortexm4::scb::unset_sleepdeep();
            }
        }

        unsafe {
            cortexm4::support::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm4::support::atomic(f)
    }
}
