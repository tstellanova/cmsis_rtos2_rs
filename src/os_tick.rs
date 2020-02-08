
/*
generated using rust-bindgen command:
` bindgen --use-core --no-layout-tests --ctypes-prefix=cty ../../cmsis_freertos_stm32h7/Drivers/CMSIS/RTOS2/Include/os_tick.h -o os_tick.rs`

and manually stripped of Darwin prefix
*/


pub type IRQHandler_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
    #[doc = " Setup OS Tick timer to generate periodic RTOS Kernel Ticks"]
    #[doc = " \\param[in]     freq         tick frequency in Hz"]
    #[doc = " \\param[in]     handler      tick IRQ handler"]
    #[doc = " \\return 0 on success, -1 on error."]
    pub fn OS_Tick_Setup(freq: u32, handler: IRQHandler_t) -> i32;
}
extern "C" {
    #[doc = " Enable OS Tick timer interrupt"]
    pub fn OS_Tick_Enable();
}
extern "C" {
    #[doc = " Disable OS Tick timer interrupt"]
    pub fn OS_Tick_Disable();
}
extern "C" {
    #[doc = " Acknowledge execution of OS Tick timer interrupt"]
    pub fn OS_Tick_AcknowledgeIRQ();
}
extern "C" {
    #[doc = " Get OS Tick timer IRQ number"]
    #[doc = " \\return OS Tick IRQ number"]
    pub fn OS_Tick_GetIRQn() -> i32;
}
extern "C" {
    #[doc = " Get OS Tick timer clock frequency"]
    #[doc = " \\return OS Tick timer clock frequency in Hz"]
    pub fn OS_Tick_GetClock() -> u32;
}
extern "C" {
    #[doc = " Get OS Tick timer interval reload value"]
    #[doc = " \\return OS Tick timer interval reload value"]
    pub fn OS_Tick_GetInterval() -> u32;
}
extern "C" {
    #[doc = " Get OS Tick timer counter value"]
    #[doc = " \\return OS Tick timer counter value"]
    pub fn OS_Tick_GetCount() -> u32;
}
extern "C" {
    #[doc = " Get OS Tick timer overflow status"]
    #[doc = " \\return OS Tick overflow status (1 - overflow, 0 - no overflow)."]
    pub fn OS_Tick_GetOverflow() -> u32;
}
