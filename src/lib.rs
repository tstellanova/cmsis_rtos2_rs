#![deny(warnings)]
#![no_std]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod cmsis_os2;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod os_tick;

pub use cmsis_os2::*;


//#[repr(C)] pub struct MsgQueue { queue_id: usize,  _private: [u8; 0] }
//#[repr(C)] pub struct Thread { thread_id: osThreadId_t,  _private: [u8; 0] }

//unsafe impl Send for MsgQueue {}
//unsafe impl Sync for MsgQueue {}

pub fn rtos_kernel_initialize() -> cmsis_os2::osStatus_t {
    unsafe {
        cmsis_os2::osKernelInitialize()
    }
}

pub fn rtos_kernel_start() -> cmsis_os2::osStatus_t {
    unsafe {
        cmsis_os2::osKernelStart()
    }
}

pub fn rtos_kernel_get_tick_freq_hz() -> u32 {
    unsafe {
        cmsis_os2::osKernelGetTickFreq()
    }
}

pub fn rtos_kernel_get_sys_timer_freq_hz() -> u32 {
    unsafe {
        cmsis_os2::osKernelGetSysTimerFreq()
    }
}

pub fn rtos_os_thread_new(func: osThreadFunc_t,  argument: *mut cty::c_void,  attr: *const osThreadAttr_t) -> osThreadId_t {
    unsafe {
        cmsis_os2::osThreadNew(func, argument, attr)
    }
}

pub fn rtos_os_thread_yield()  -> cmsis_os2::osStatus_t {
    unsafe {
        cmsis_os2::osThreadYield()
    }
}

pub fn rtos_os_delay(ticks: u32) -> cmsis_os2::osStatus_t {
    unsafe {
        cmsis_os2::osDelay(ticks)
    }
}

pub fn rtos_os_timer_new(func: osTimerFunc_t, timer_type: osTimerType_t,  argument: *mut cty::c_void,  attr: *const osTimerAttr_t) -> osTimerId_t {
    unsafe {
        cmsis_os2::osTimerNew(func, timer_type, argument, attr)
    }
}

pub fn rtos_os_timer_start(timer_id: osTimerId_t, ticks: u32) -> osStatus_t {
    unsafe {
        cmsis_os2::osTimerStart(timer_id, ticks)
    }
}

pub fn rtos_os_timer_stop(timer_id: osTimerId_t) -> osStatus_t {
    unsafe {
        cmsis_os2::osTimerStop(timer_id)
    }
}


pub fn rtos_os_msg_queue_new(msg_count: u32,
                             msg_size: u32,
                             attr: *const osMessageQueueAttr_t) -> osMessageQueueId_t {
    unsafe {
        cmsis_os2::osMessageQueueNew(msg_count, msg_size, attr)
    }
}

pub fn rtos_os_msg_queue_put(mq_id: osMessageQueueId_t,
                             msg_ptr: *const cty::c_void,
                             msg_prio: u8,
                             timeout: u32) -> osStatus_t {
    unsafe  {
        cmsis_os2::osMessageQueuePut(mq_id, msg_ptr, msg_prio, timeout)
    }

}

pub fn rtos_os_msg_queue_get(mq_id: osMessageQueueId_t,
                             msg_ptr: *mut cty::c_void,
                             msg_prio: *mut u8,
                             timeout: u32) -> osStatus_t {
    unsafe {
        cmsis_os2::osMessageQueueGet(mq_id, msg_ptr, msg_prio, timeout)
    }
}




