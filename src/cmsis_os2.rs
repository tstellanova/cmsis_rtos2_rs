
/*
generated using rust-bindgen command:
` bindgen --use-core --no-layout-tests --ctypes-prefix=cty /path/to/cmsis_os2.h -o cmsis_os2.rs`

and manually stripped of Darwin prefix
*/


pub const osWaitForever: u32 = 4294967295;
pub const osFlagsWaitAny: u32 = 0;
pub const osFlagsWaitAll: u32 = 1;
pub const osFlagsNoClear: u32 = 2;
pub const osFlagsError: u32 = 2147483648;
pub const osFlagsErrorUnknown: u32 = 4294967295;
pub const osFlagsErrorTimeout: u32 = 4294967294;
pub const osFlagsErrorResource: u32 = 4294967293;
pub const osFlagsErrorParameter: u32 = 4294967292;
pub const osFlagsErrorISR: u32 = 4294967290;
pub const osThreadDetached: u32 = 0;
pub const osThreadJoinable: u32 = 1;
pub const osMutexRecursive: u32 = 1;
pub const osMutexPrioInherit: u32 = 2;
pub const osMutexRobust: u32 = 8;

#[doc = " Version information."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osVersion_t {
    #[doc = "< API version (major.minor.rev: mmnnnrrrr dec)."]
    pub api: u32,
    #[doc = "< Kernel version (major.minor.rev: mmnnnrrrr dec)."]
    pub kernel: u32,
}

#[doc = "< Inactive."]
pub const osKernelState_t_osKernelInactive: osKernelState_t = 0;
#[doc = "< Ready."]
pub const osKernelState_t_osKernelReady: osKernelState_t = 1;
#[doc = "< Running."]
pub const osKernelState_t_osKernelRunning: osKernelState_t = 2;
#[doc = "< Locked."]
pub const osKernelState_t_osKernelLocked: osKernelState_t = 3;
#[doc = "< Suspended."]
pub const osKernelState_t_osKernelSuspended: osKernelState_t = 4;
#[doc = "< Error."]
pub const osKernelState_t_osKernelError: osKernelState_t = -1;
#[doc = "< Prevents enum down-size compiler optimization."]
pub const osKernelState_t_osKernelReserved: osKernelState_t = 2147483647;
#[doc = " Kernel state."]
pub type osKernelState_t = i32;
#[doc = "< Inactive."]
pub const osThreadState_t_osThreadInactive: osThreadState_t = 0;
#[doc = "< Ready."]
pub const osThreadState_t_osThreadReady: osThreadState_t = 1;
#[doc = "< Running."]
pub const osThreadState_t_osThreadRunning: osThreadState_t = 2;
#[doc = "< Blocked."]
pub const osThreadState_t_osThreadBlocked: osThreadState_t = 3;
#[doc = "< Terminated."]
pub const osThreadState_t_osThreadTerminated: osThreadState_t = 4;
#[doc = "< Error."]
pub const osThreadState_t_osThreadError: osThreadState_t = -1;
#[doc = "< Prevents enum down-size compiler optimization."]
pub const osThreadState_t_osThreadReserved: osThreadState_t = 2147483647;
#[doc = " Thread state."]
pub type osThreadState_t = i32;
#[doc = "< No priority (not initialized)."]
pub const osPriority_t_osPriorityNone: osPriority_t = 0;
#[doc = "< Reserved for Idle thread."]
pub const osPriority_t_osPriorityIdle: osPriority_t = 1;
#[doc = "< Priority: low"]
pub const osPriority_t_osPriorityLow: osPriority_t = 8;
#[doc = "< Priority: low + 1"]
pub const osPriority_t_osPriorityLow1: osPriority_t = 9;
#[doc = "< Priority: low + 2"]
pub const osPriority_t_osPriorityLow2: osPriority_t = 10;
#[doc = "< Priority: low + 3"]
pub const osPriority_t_osPriorityLow3: osPriority_t = 11;
#[doc = "< Priority: low + 4"]
pub const osPriority_t_osPriorityLow4: osPriority_t = 12;
#[doc = "< Priority: low + 5"]
pub const osPriority_t_osPriorityLow5: osPriority_t = 13;
#[doc = "< Priority: low + 6"]
pub const osPriority_t_osPriorityLow6: osPriority_t = 14;
#[doc = "< Priority: low + 7"]
pub const osPriority_t_osPriorityLow7: osPriority_t = 15;
#[doc = "< Priority: below normal"]
pub const osPriority_t_osPriorityBelowNormal: osPriority_t = 16;
#[doc = "< Priority: below normal + 1"]
pub const osPriority_t_osPriorityBelowNormal1: osPriority_t = 17;
#[doc = "< Priority: below normal + 2"]
pub const osPriority_t_osPriorityBelowNormal2: osPriority_t = 18;
#[doc = "< Priority: below normal + 3"]
pub const osPriority_t_osPriorityBelowNormal3: osPriority_t = 19;
#[doc = "< Priority: below normal + 4"]
pub const osPriority_t_osPriorityBelowNormal4: osPriority_t = 20;
#[doc = "< Priority: below normal + 5"]
pub const osPriority_t_osPriorityBelowNormal5: osPriority_t = 21;
#[doc = "< Priority: below normal + 6"]
pub const osPriority_t_osPriorityBelowNormal6: osPriority_t = 22;
#[doc = "< Priority: below normal + 7"]
pub const osPriority_t_osPriorityBelowNormal7: osPriority_t = 23;
#[doc = "< Priority: normal"]
pub const osPriority_t_osPriorityNormal: osPriority_t = 24;
#[doc = "< Priority: normal + 1"]
pub const osPriority_t_osPriorityNormal1: osPriority_t = 25;
#[doc = "< Priority: normal + 2"]
pub const osPriority_t_osPriorityNormal2: osPriority_t = 26;
#[doc = "< Priority: normal + 3"]
pub const osPriority_t_osPriorityNormal3: osPriority_t = 27;
#[doc = "< Priority: normal + 4"]
pub const osPriority_t_osPriorityNormal4: osPriority_t = 28;
#[doc = "< Priority: normal + 5"]
pub const osPriority_t_osPriorityNormal5: osPriority_t = 29;
#[doc = "< Priority: normal + 6"]
pub const osPriority_t_osPriorityNormal6: osPriority_t = 30;
#[doc = "< Priority: normal + 7"]
pub const osPriority_t_osPriorityNormal7: osPriority_t = 31;
#[doc = "< Priority: above normal"]
pub const osPriority_t_osPriorityAboveNormal: osPriority_t = 32;
#[doc = "< Priority: above normal + 1"]
pub const osPriority_t_osPriorityAboveNormal1: osPriority_t = 33;
#[doc = "< Priority: above normal + 2"]
pub const osPriority_t_osPriorityAboveNormal2: osPriority_t = 34;
#[doc = "< Priority: above normal + 3"]
pub const osPriority_t_osPriorityAboveNormal3: osPriority_t = 35;
#[doc = "< Priority: above normal + 4"]
pub const osPriority_t_osPriorityAboveNormal4: osPriority_t = 36;
#[doc = "< Priority: above normal + 5"]
pub const osPriority_t_osPriorityAboveNormal5: osPriority_t = 37;
#[doc = "< Priority: above normal + 6"]
pub const osPriority_t_osPriorityAboveNormal6: osPriority_t = 38;
#[doc = "< Priority: above normal + 7"]
pub const osPriority_t_osPriorityAboveNormal7: osPriority_t = 39;
#[doc = "< Priority: high"]
pub const osPriority_t_osPriorityHigh: osPriority_t = 40;
#[doc = "< Priority: high + 1"]
pub const osPriority_t_osPriorityHigh1: osPriority_t = 41;
#[doc = "< Priority: high + 2"]
pub const osPriority_t_osPriorityHigh2: osPriority_t = 42;
#[doc = "< Priority: high + 3"]
pub const osPriority_t_osPriorityHigh3: osPriority_t = 43;
#[doc = "< Priority: high + 4"]
pub const osPriority_t_osPriorityHigh4: osPriority_t = 44;
#[doc = "< Priority: high + 5"]
pub const osPriority_t_osPriorityHigh5: osPriority_t = 45;
#[doc = "< Priority: high + 6"]
pub const osPriority_t_osPriorityHigh6: osPriority_t = 46;
#[doc = "< Priority: high + 7"]
pub const osPriority_t_osPriorityHigh7: osPriority_t = 47;
#[doc = "< Priority: realtime"]
pub const osPriority_t_osPriorityRealtime: osPriority_t = 48;
#[doc = "< Priority: realtime + 1"]
pub const osPriority_t_osPriorityRealtime1: osPriority_t = 49;
#[doc = "< Priority: realtime + 2"]
pub const osPriority_t_osPriorityRealtime2: osPriority_t = 50;
#[doc = "< Priority: realtime + 3"]
pub const osPriority_t_osPriorityRealtime3: osPriority_t = 51;
#[doc = "< Priority: realtime + 4"]
pub const osPriority_t_osPriorityRealtime4: osPriority_t = 52;
#[doc = "< Priority: realtime + 5"]
pub const osPriority_t_osPriorityRealtime5: osPriority_t = 53;
#[doc = "< Priority: realtime + 6"]
pub const osPriority_t_osPriorityRealtime6: osPriority_t = 54;
#[doc = "< Priority: realtime + 7"]
pub const osPriority_t_osPriorityRealtime7: osPriority_t = 55;
#[doc = "< Reserved for ISR deferred thread."]
pub const osPriority_t_osPriorityISR: osPriority_t = 56;
#[doc = "< System cannot determine priority or illegal priority."]
pub const osPriority_t_osPriorityError: osPriority_t = -1;
#[doc = "< Prevents enum down-size compiler optimization."]
pub const osPriority_t_osPriorityReserved: osPriority_t = 2147483647;
#[doc = " Priority values."]
pub type osPriority_t = i32;
#[doc = " Entry point of a thread."]
pub type osThreadFunc_t = ::core::option::Option<unsafe extern "C" fn(argument: *mut cty::c_void)>;
#[doc = " Timer callback function."]
pub type osTimerFunc_t = ::core::option::Option<unsafe extern "C" fn(argument: *mut cty::c_void)>;
#[doc = "< One-shot timer."]
pub const osTimerType_t_osTimerOnce: osTimerType_t = 0;
#[doc = "< Repeating timer."]
pub const osTimerType_t_osTimerPeriodic: osTimerType_t = 1;
#[doc = " Timer type."]
pub type osTimerType_t = u32;
#[doc = "< Operation completed successfully."]
pub const osStatus_t_osOK: osStatus_t = 0;
#[doc = "< Unspecified RTOS error: run-time error but no other error message fits."]
pub const osStatus_t_osError: osStatus_t = -1;
#[doc = "< Operation not completed within the timeout period."]
pub const osStatus_t_osErrorTimeout: osStatus_t = -2;
#[doc = "< Resource not available."]
pub const osStatus_t_osErrorResource: osStatus_t = -3;
#[doc = "< Parameter error."]
pub const osStatus_t_osErrorParameter: osStatus_t = -4;
#[doc = "< System is out of memory: it was impossible to allocate or reserve memory for the operation."]
pub const osStatus_t_osErrorNoMemory: osStatus_t = -5;
#[doc = "< Not allowed in ISR context: the function cannot be called from interrupt service routines."]
pub const osStatus_t_osErrorISR: osStatus_t = -6;
#[doc = "< Prevents enum down-size compiler optimization."]
pub const osStatus_t_osStatusReserved: osStatus_t = 2147483647;
#[doc = " Status code values returned by CMSIS-RTOS functions."]
pub type osStatus_t = i32;
#[doc = " \\details Thread ID identifies the thread."]
pub type osThreadId_t = *mut cty::c_void;
#[doc = " \\details Timer ID identifies the timer."]
pub type osTimerId_t = *mut cty::c_void;
#[doc = " \\details Event Flags ID identifies the event flags."]
pub type osEventFlagsId_t = *mut cty::c_void;
#[doc = " \\details Mutex ID identifies the mutex."]
pub type osMutexId_t = *mut cty::c_void;
#[doc = " \\details Semaphore ID identifies the semaphore."]
pub type osSemaphoreId_t = *mut cty::c_void;
#[doc = " \\details Memory Pool ID identifies the memory pool."]
pub type osMemoryPoolId_t = *mut cty::c_void;
#[doc = " \\details Message Queue ID identifies the message queue."]
pub type osMessageQueueId_t = *mut cty::c_void;
#[doc = " \\details Data type that identifies secure software modules called by a process."]
pub type TZ_ModuleId_t = u32;
#[doc = " Attributes structure for thread."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osThreadAttr_t {
    #[doc = "< name of the thread"]
    pub name: *const cty::c_char,
    #[doc = "< attribute bits"]
    pub attr_bits: u32,
    #[doc = "< memory for control block"]
    pub cb_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for control block"]
    pub cb_size: u32,
    #[doc = "< memory for stack"]
    pub stack_mem: *mut cty::c_void,
    #[doc = "< size of stack"]
    pub stack_size: u32,
    #[doc = "< initial thread priority (default: osPriorityNormal)"]
    pub priority: osPriority_t,
    #[doc = "< TrustZone module identifier"]
    pub tz_module: TZ_ModuleId_t,
    #[doc = "< reserved (must be 0)"]
    pub reserved: u32,
}
#[doc = " Attributes structure for timer."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osTimerAttr_t {
    #[doc = "< name of the timer"]
    pub name: *const cty::c_char,
    #[doc = "< attribute bits"]
    pub attr_bits: u32,
    #[doc = "< memory for control block"]
    pub cb_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for control block"]
    pub cb_size: u32,
}
#[doc = " Attributes structure for event flags."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osEventFlagsAttr_t {
    #[doc = "< name of the event flags"]
    pub name: *const cty::c_char,
    #[doc = "< attribute bits"]
    pub attr_bits: u32,
    #[doc = "< memory for control block"]
    pub cb_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for control block"]
    pub cb_size: u32,
}
#[doc = " Attributes structure for mutex."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osMutexAttr_t {
    #[doc = "< name of the mutex"]
    pub name: *const cty::c_char,
    #[doc = "< attribute bits"]
    pub attr_bits: u32,
    #[doc = "< memory for control block"]
    pub cb_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for control block"]
    pub cb_size: u32,
}
#[doc = " Attributes structure for semaphore."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osSemaphoreAttr_t {
    #[doc = "< name of the semaphore"]
    pub name: *const cty::c_char,
    #[doc = "< attribute bits"]
    pub attr_bits: u32,
    #[doc = "< memory for control block"]
    pub cb_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for control block"]
    pub cb_size: u32,
}
#[doc = " Attributes structure for memory pool."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osMemoryPoolAttr_t {
    #[doc = "< name of the memory pool"]
    pub name: *const cty::c_char,
    #[doc = "< attribute bits"]
    pub attr_bits: u32,
    #[doc = "< memory for control block"]
    pub cb_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for control block"]
    pub cb_size: u32,
    #[doc = "< memory for data storage"]
    pub mp_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for data storage"]
    pub mp_size: u32,
}
#[doc = " Attributes structure for message queue."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osMessageQueueAttr_t {
    #[doc = "< name of the message queue"]
    pub name: *const cty::c_char,
    #[doc = "< attribute bits"]
    pub attr_bits: u32,
    #[doc = "< memory for control block"]
    pub cb_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for control block"]
    pub cb_size: u32,
    #[doc = "< memory for data storage"]
    pub mq_mem: *mut cty::c_void,
    #[doc = "< size of provided memory for data storage"]
    pub mq_size: u32,
}
extern "C" {
    #[doc = " Initialize the RTOS Kernel."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osKernelInitialize() -> osStatus_t;
}
extern "C" {
    #[doc = "  Get RTOS Kernel Information."]
    #[doc = " \\param[out]    version       pointer to buffer for retrieving version information."]
    #[doc = " \\param[out]    id_buf        pointer to buffer for retrieving kernel identification string."]
    #[doc = " \\param[in]     id_size       size of buffer for kernel identification string."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osKernelGetInfo(
        version: *mut osVersion_t,
        id_buf: *mut cty::c_char,
        id_size: u32,
    ) -> osStatus_t;
}
extern "C" {
    #[doc = " Get the current RTOS Kernel state."]
    #[doc = " \\return current RTOS Kernel state."]
    pub fn osKernelGetState() -> osKernelState_t;
}
extern "C" {
    #[doc = " Start the RTOS Kernel scheduler."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osKernelStart() -> osStatus_t;
}
extern "C" {
    #[doc = " Lock the RTOS Kernel scheduler."]
    #[doc = " \\return previous lock state (1 - locked, 0 - not locked, error code if negative)."]
    pub fn osKernelLock() -> i32;
}
extern "C" {
    #[doc = " Unlock the RTOS Kernel scheduler."]
    #[doc = " \\return previous lock state (1 - locked, 0 - not locked, error code if negative)."]
    pub fn osKernelUnlock() -> i32;
}
extern "C" {
    #[doc = " Restore the RTOS Kernel scheduler lock state."]
    #[doc = " \\param[in]     lock          lock state obtained by \\ref osKernelLock or \\ref osKernelUnlock."]
    #[doc = " \\return new lock state (1 - locked, 0 - not locked, error code if negative)."]
    pub fn osKernelRestoreLock(lock: i32) -> i32;
}
extern "C" {
    #[doc = " Suspend the RTOS Kernel scheduler."]
    #[doc = " \\return time in ticks, for how long the system can sleep or power-down."]
    pub fn osKernelSuspend() -> u32;
}
extern "C" {
    #[doc = " Resume the RTOS Kernel scheduler."]
    #[doc = " \\param[in]     sleep_ticks   time in ticks for how long the system was in sleep or power-down mode."]
    pub fn osKernelResume(sleep_ticks: u32);
}
extern "C" {
    #[doc = " Get the RTOS kernel tick count."]
    #[doc = " \\return RTOS kernel current tick count."]
    pub fn osKernelGetTickCount() -> u32;
}
extern "C" {
    #[doc = " Get the RTOS kernel tick frequency."]
    #[doc = " \\return frequency of the kernel tick in hertz, i.e. kernel ticks per second."]
    pub fn osKernelGetTickFreq() -> u32;
}
extern "C" {
    #[doc = " Get the RTOS kernel system timer count."]
    #[doc = " \\return RTOS kernel current system timer count as 32-bit value."]
    pub fn osKernelGetSysTimerCount() -> u32;
}
extern "C" {
    #[doc = " Get the RTOS kernel system timer frequency."]
    #[doc = " \\return frequency of the system timer in hertz, i.e. timer ticks per second."]
    pub fn osKernelGetSysTimerFreq() -> u32;
}
extern "C" {
    #[doc = " Create a thread and add it to Active Threads."]
    #[doc = " \\param[in]     func          thread function."]
    #[doc = " \\param[in]     argument      pointer that is passed to the thread function as start argument."]
    #[doc = " \\param[in]     attr          thread attributes; NULL: default values."]
    #[doc = " \\return thread ID for reference by other functions or NULL in case of error."]
    pub fn osThreadNew(
        func: osThreadFunc_t,
        argument: *mut cty::c_void,
        attr: *const osThreadAttr_t,
    ) -> osThreadId_t;
}
extern "C" {
    #[doc = " Get name of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return name as null-terminated string."]
    pub fn osThreadGetName(thread_id: osThreadId_t) -> *const cty::c_char;
}
extern "C" {
    #[doc = " Return the thread ID of the current running thread."]
    #[doc = " \\return thread ID for reference by other functions or NULL in case of error."]
    pub fn osThreadGetId() -> osThreadId_t;
}
extern "C" {
    #[doc = " Get current thread state of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return current thread state of the specified thread."]
    pub fn osThreadGetState(thread_id: osThreadId_t) -> osThreadState_t;
}
extern "C" {
    #[doc = " Get stack size of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return stack size in bytes."]
    pub fn osThreadGetStackSize(thread_id: osThreadId_t) -> u32;
}
extern "C" {
    #[doc = " Get available stack space of a thread based on stack watermark recording during execution."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return remaining stack space in bytes."]
    pub fn osThreadGetStackSpace(thread_id: osThreadId_t) -> u32;
}
extern "C" {
    #[doc = " Change priority of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\param[in]     priority      new priority value for the thread function."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osThreadSetPriority(thread_id: osThreadId_t, priority: osPriority_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Get current priority of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return current priority value of the specified thread."]
    pub fn osThreadGetPriority(thread_id: osThreadId_t) -> osPriority_t;
}
extern "C" {
    #[doc = " Pass control to next thread that is in state \\b READY."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osThreadYield() -> osStatus_t;
}
extern "C" {
    #[doc = " Suspend execution of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osThreadSuspend(thread_id: osThreadId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Resume execution of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osThreadResume(thread_id: osThreadId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Detach a thread (thread storage can be reclaimed when thread terminates)."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osThreadDetach(thread_id: osThreadId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Wait for specified thread to terminate."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osThreadJoin(thread_id: osThreadId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Terminate execution of current running thread."]
    pub fn osThreadExit();
}
extern "C" {
    #[doc = " Terminate execution of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osThreadTerminate(thread_id: osThreadId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Get number of active threads."]
    #[doc = " \\return number of active threads."]
    pub fn osThreadGetCount() -> u32;
}
extern "C" {
    #[doc = " Enumerate active threads."]
    #[doc = " \\param[out]    thread_array  pointer to array for retrieving thread IDs."]
    #[doc = " \\param[in]     array_items   maximum number of items in array for retrieving thread IDs."]
    #[doc = " \\return number of enumerated threads."]
    pub fn osThreadEnumerate(thread_array: *mut osThreadId_t, array_items: u32) -> u32;
}
extern "C" {
    #[doc = " Set the specified Thread Flags of a thread."]
    #[doc = " \\param[in]     thread_id     thread ID obtained by \\ref osThreadNew or \\ref osThreadGetId."]
    #[doc = " \\param[in]     flags         specifies the flags of the thread that shall be set."]
    #[doc = " \\return thread flags after setting or error code if highest bit set."]
    pub fn osThreadFlagsSet(thread_id: osThreadId_t, flags: u32) -> u32;
}
extern "C" {
    #[doc = " Clear the specified Thread Flags of current running thread."]
    #[doc = " \\param[in]     flags         specifies the flags of the thread that shall be cleared."]
    #[doc = " \\return thread flags before clearing or error code if highest bit set."]
    pub fn osThreadFlagsClear(flags: u32) -> u32;
}
extern "C" {
    #[doc = " Get the current Thread Flags of current running thread."]
    #[doc = " \\return current thread flags."]
    pub fn osThreadFlagsGet() -> u32;
}
extern "C" {
    #[doc = " Wait for one or more Thread Flags of the current running thread to become signaled."]
    #[doc = " \\param[in]     flags         specifies the flags to wait for."]
    #[doc = " \\param[in]     options       specifies flags options (osFlagsXxxx)."]
    #[doc = " \\param[in]     timeout       \\ref CMSIS_RTOS_TimeOutValue or 0 in case of no time-out."]
    #[doc = " \\return thread flags before clearing or error code if highest bit set."]
    pub fn osThreadFlagsWait(flags: u32, options: u32, timeout: u32) -> u32;
}
extern "C" {
    #[doc = " Wait for Timeout (Time Delay)."]
    #[doc = " \\param[in]     ticks         \\ref CMSIS_RTOS_TimeOutValue \"time ticks\" value"]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osDelay(ticks: u32) -> osStatus_t;
}
extern "C" {
    #[doc = " Wait until specified time."]
    #[doc = " \\param[in]     ticks         absolute time in ticks"]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osDelayUntil(ticks: u32) -> osStatus_t;
}
extern "C" {
    #[doc = " Create and Initialize a timer."]
    #[doc = " \\param[in]     func          function pointer to callback function."]
    #[doc = " \\param[in]     type          \\ref osTimerOnce for one-shot or \\ref osTimerPeriodic for periodic behavior."]
    #[doc = " \\param[in]     argument      argument to the timer callback function."]
    #[doc = " \\param[in]     attr          timer attributes; NULL: default values."]
    #[doc = " \\return timer ID for reference by other functions or NULL in case of error."]
    pub fn osTimerNew(
        func: osTimerFunc_t,
        type_: osTimerType_t,
        argument: *mut cty::c_void,
        attr: *const osTimerAttr_t,
    ) -> osTimerId_t;
}
extern "C" {
    #[doc = " Get name of a timer."]
    #[doc = " \\param[in]     timer_id      timer ID obtained by \\ref osTimerNew."]
    #[doc = " \\return name as null-terminated string."]
    pub fn osTimerGetName(timer_id: osTimerId_t) -> *const cty::c_char;
}
extern "C" {
    #[doc = " Start or restart a timer."]
    #[doc = " \\param[in]     timer_id      timer ID obtained by \\ref osTimerNew."]
    #[doc = " \\param[in]     ticks         \\ref CMSIS_RTOS_TimeOutValue \"time ticks\" value of the timer."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osTimerStart(timer_id: osTimerId_t, ticks: u32) -> osStatus_t;
}
extern "C" {
    #[doc = " Stop a timer."]
    #[doc = " \\param[in]     timer_id      timer ID obtained by \\ref osTimerNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osTimerStop(timer_id: osTimerId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Check if a timer is running."]
    #[doc = " \\param[in]     timer_id      timer ID obtained by \\ref osTimerNew."]
    #[doc = " \\return 0 not running, 1 running."]
    pub fn osTimerIsRunning(timer_id: osTimerId_t) -> u32;
}
extern "C" {
    #[doc = " Delete a timer."]
    #[doc = " \\param[in]     timer_id      timer ID obtained by \\ref osTimerNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osTimerDelete(timer_id: osTimerId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Create and Initialize an Event Flags object."]
    #[doc = " \\param[in]     attr          event flags attributes; NULL: default values."]
    #[doc = " \\return event flags ID for reference by other functions or NULL in case of error."]
    pub fn osEventFlagsNew(attr: *const osEventFlagsAttr_t) -> osEventFlagsId_t;
}
extern "C" {
    #[doc = " Get name of an Event Flags object."]
    #[doc = " \\param[in]     ef_id         event flags ID obtained by \\ref osEventFlagsNew."]
    #[doc = " \\return name as null-terminated string."]
    pub fn osEventFlagsGetName(ef_id: osEventFlagsId_t) -> *const cty::c_char;
}
extern "C" {
    #[doc = " Set the specified Event Flags."]
    #[doc = " \\param[in]     ef_id         event flags ID obtained by \\ref osEventFlagsNew."]
    #[doc = " \\param[in]     flags         specifies the flags that shall be set."]
    #[doc = " \\return event flags after setting or error code if highest bit set."]
    pub fn osEventFlagsSet(ef_id: osEventFlagsId_t, flags: u32) -> u32;
}
extern "C" {
    #[doc = " Clear the specified Event Flags."]
    #[doc = " \\param[in]     ef_id         event flags ID obtained by \\ref osEventFlagsNew."]
    #[doc = " \\param[in]     flags         specifies the flags that shall be cleared."]
    #[doc = " \\return event flags before clearing or error code if highest bit set."]
    pub fn osEventFlagsClear(ef_id: osEventFlagsId_t, flags: u32) -> u32;
}
extern "C" {
    #[doc = " Get the current Event Flags."]
    #[doc = " \\param[in]     ef_id         event flags ID obtained by \\ref osEventFlagsNew."]
    #[doc = " \\return current event flags."]
    pub fn osEventFlagsGet(ef_id: osEventFlagsId_t) -> u32;
}
extern "C" {
    #[doc = " Wait for one or more Event Flags to become signaled."]
    #[doc = " \\param[in]     ef_id         event flags ID obtained by \\ref osEventFlagsNew."]
    #[doc = " \\param[in]     flags         specifies the flags to wait for."]
    #[doc = " \\param[in]     options       specifies flags options (osFlagsXxxx)."]
    #[doc = " \\param[in]     timeout       \\ref CMSIS_RTOS_TimeOutValue or 0 in case of no time-out."]
    #[doc = " \\return event flags before clearing or error code if highest bit set."]
    pub fn osEventFlagsWait(ef_id: osEventFlagsId_t, flags: u32, options: u32, timeout: u32)
        -> u32;
}
extern "C" {
    #[doc = " Delete an Event Flags object."]
    #[doc = " \\param[in]     ef_id         event flags ID obtained by \\ref osEventFlagsNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osEventFlagsDelete(ef_id: osEventFlagsId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Create and Initialize a Mutex object."]
    #[doc = " \\param[in]     attr          mutex attributes; NULL: default values."]
    #[doc = " \\return mutex ID for reference by other functions or NULL in case of error."]
    pub fn osMutexNew(attr: *const osMutexAttr_t) -> osMutexId_t;
}
extern "C" {
    #[doc = " Get name of a Mutex object."]
    #[doc = " \\param[in]     mutex_id      mutex ID obtained by \\ref osMutexNew."]
    #[doc = " \\return name as null-terminated string."]
    pub fn osMutexGetName(mutex_id: osMutexId_t) -> *const cty::c_char;
}
extern "C" {
    #[doc = " Acquire a Mutex or timeout if it is locked."]
    #[doc = " \\param[in]     mutex_id      mutex ID obtained by \\ref osMutexNew."]
    #[doc = " \\param[in]     timeout       \\ref CMSIS_RTOS_TimeOutValue or 0 in case of no time-out."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMutexAcquire(mutex_id: osMutexId_t, timeout: u32) -> osStatus_t;
}
extern "C" {
    #[doc = " Release a Mutex that was acquired by \\ref osMutexAcquire."]
    #[doc = " \\param[in]     mutex_id      mutex ID obtained by \\ref osMutexNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMutexRelease(mutex_id: osMutexId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Get Thread which owns a Mutex object."]
    #[doc = " \\param[in]     mutex_id      mutex ID obtained by \\ref osMutexNew."]
    #[doc = " \\return thread ID of owner thread or NULL when mutex was not acquired."]
    pub fn osMutexGetOwner(mutex_id: osMutexId_t) -> osThreadId_t;
}
extern "C" {
    #[doc = " Delete a Mutex object."]
    #[doc = " \\param[in]     mutex_id      mutex ID obtained by \\ref osMutexNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMutexDelete(mutex_id: osMutexId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Create and Initialize a Semaphore object."]
    #[doc = " \\param[in]     max_count     maximum number of available tokens."]
    #[doc = " \\param[in]     initial_count initial number of available tokens."]
    #[doc = " \\param[in]     attr          semaphore attributes; NULL: default values."]
    #[doc = " \\return semaphore ID for reference by other functions or NULL in case of error."]
    pub fn osSemaphoreNew(
        max_count: u32,
        initial_count: u32,
        attr: *const osSemaphoreAttr_t,
    ) -> osSemaphoreId_t;
}
extern "C" {
    #[doc = " Get name of a Semaphore object."]
    #[doc = " \\param[in]     semaphore_id  semaphore ID obtained by \\ref osSemaphoreNew."]
    #[doc = " \\return name as null-terminated string."]
    pub fn osSemaphoreGetName(semaphore_id: osSemaphoreId_t) -> *const cty::c_char;
}
extern "C" {
    #[doc = " Acquire a Semaphore token or timeout if no tokens are available."]
    #[doc = " \\param[in]     semaphore_id  semaphore ID obtained by \\ref osSemaphoreNew."]
    #[doc = " \\param[in]     timeout       \\ref CMSIS_RTOS_TimeOutValue or 0 in case of no time-out."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osSemaphoreAcquire(semaphore_id: osSemaphoreId_t, timeout: u32) -> osStatus_t;
}
extern "C" {
    #[doc = " Release a Semaphore token up to the initial maximum count."]
    #[doc = " \\param[in]     semaphore_id  semaphore ID obtained by \\ref osSemaphoreNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osSemaphoreRelease(semaphore_id: osSemaphoreId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Get current Semaphore token count."]
    #[doc = " \\param[in]     semaphore_id  semaphore ID obtained by \\ref osSemaphoreNew."]
    #[doc = " \\return number of tokens available."]
    pub fn osSemaphoreGetCount(semaphore_id: osSemaphoreId_t) -> u32;
}
extern "C" {
    #[doc = " Delete a Semaphore object."]
    #[doc = " \\param[in]     semaphore_id  semaphore ID obtained by \\ref osSemaphoreNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osSemaphoreDelete(semaphore_id: osSemaphoreId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Create and Initialize a Memory Pool object."]
    #[doc = " \\param[in]     block_count   maximum number of memory blocks in memory pool."]
    #[doc = " \\param[in]     block_size    memory block size in bytes."]
    #[doc = " \\param[in]     attr          memory pool attributes; NULL: default values."]
    #[doc = " \\return memory pool ID for reference by other functions or NULL in case of error."]
    pub fn osMemoryPoolNew(
        block_count: u32,
        block_size: u32,
        attr: *const osMemoryPoolAttr_t,
    ) -> osMemoryPoolId_t;
}
extern "C" {
    #[doc = " Get name of a Memory Pool object."]
    #[doc = " \\param[in]     mp_id         memory pool ID obtained by \\ref osMemoryPoolNew."]
    #[doc = " \\return name as null-terminated string."]
    pub fn osMemoryPoolGetName(mp_id: osMemoryPoolId_t) -> *const cty::c_char;
}
extern "C" {
    #[doc = " Allocate a memory block from a Memory Pool."]
    #[doc = " \\param[in]     mp_id         memory pool ID obtained by \\ref osMemoryPoolNew."]
    #[doc = " \\param[in]     timeout       \\ref CMSIS_RTOS_TimeOutValue or 0 in case of no time-out."]
    #[doc = " \\return address of the allocated memory block or NULL in case of no memory is available."]
    pub fn osMemoryPoolAlloc(mp_id: osMemoryPoolId_t, timeout: u32) -> *mut cty::c_void;
}
extern "C" {
    #[doc = " Return an allocated memory block back to a Memory Pool."]
    #[doc = " \\param[in]     mp_id         memory pool ID obtained by \\ref osMemoryPoolNew."]
    #[doc = " \\param[in]     block         address of the allocated memory block to be returned to the memory pool."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMemoryPoolFree(mp_id: osMemoryPoolId_t, block: *mut cty::c_void) -> osStatus_t;
}
extern "C" {
    #[doc = " Get maximum number of memory blocks in a Memory Pool."]
    #[doc = " \\param[in]     mp_id         memory pool ID obtained by \\ref osMemoryPoolNew."]
    #[doc = " \\return maximum number of memory blocks."]
    pub fn osMemoryPoolGetCapacity(mp_id: osMemoryPoolId_t) -> u32;
}
extern "C" {
    #[doc = " Get memory block size in a Memory Pool."]
    #[doc = " \\param[in]     mp_id         memory pool ID obtained by \\ref osMemoryPoolNew."]
    #[doc = " \\return memory block size in bytes."]
    pub fn osMemoryPoolGetBlockSize(mp_id: osMemoryPoolId_t) -> u32;
}
extern "C" {
    #[doc = " Get number of memory blocks used in a Memory Pool."]
    #[doc = " \\param[in]     mp_id         memory pool ID obtained by \\ref osMemoryPoolNew."]
    #[doc = " \\return number of memory blocks used."]
    pub fn osMemoryPoolGetCount(mp_id: osMemoryPoolId_t) -> u32;
}
extern "C" {
    #[doc = " Get number of memory blocks available in a Memory Pool."]
    #[doc = " \\param[in]     mp_id         memory pool ID obtained by \\ref osMemoryPoolNew."]
    #[doc = " \\return number of memory blocks available."]
    pub fn osMemoryPoolGetSpace(mp_id: osMemoryPoolId_t) -> u32;
}
extern "C" {
    #[doc = " Delete a Memory Pool object."]
    #[doc = " \\param[in]     mp_id         memory pool ID obtained by \\ref osMemoryPoolNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMemoryPoolDelete(mp_id: osMemoryPoolId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Create and Initialize a Message Queue object."]
    #[doc = " \\param[in]     msg_count     maximum number of messages in queue."]
    #[doc = " \\param[in]     msg_size      maximum message size in bytes."]
    #[doc = " \\param[in]     attr          message queue attributes; NULL: default values."]
    #[doc = " \\return message queue ID for reference by other functions or NULL in case of error."]
    pub fn osMessageQueueNew(
        msg_count: u32,
        msg_size: u32,
        attr: *const osMessageQueueAttr_t,
    ) -> osMessageQueueId_t;
}
extern "C" {
    #[doc = " Get name of a Message Queue object."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\return name as null-terminated string."]
    pub fn osMessageQueueGetName(mq_id: osMessageQueueId_t) -> *const cty::c_char;
}
extern "C" {
    #[doc = " Put a Message into a Queue or timeout if Queue is full."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\param[in]     msg_ptr       pointer to buffer with message to put into a queue."]
    #[doc = " \\param[in]     msg_prio      message priority."]
    #[doc = " \\param[in]     timeout       \\ref CMSIS_RTOS_TimeOutValue or 0 in case of no time-out."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMessageQueuePut(
        mq_id: osMessageQueueId_t,
        msg_ptr: *const cty::c_void,
        msg_prio: u8,
        timeout: u32,
    ) -> osStatus_t;
}
extern "C" {
    #[doc = " Get a Message from a Queue or timeout if Queue is empty."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\param[out]    msg_ptr       pointer to buffer for message to get from a queue."]
    #[doc = " \\param[out]    msg_prio      pointer to buffer for message priority or NULL."]
    #[doc = " \\param[in]     timeout       \\ref CMSIS_RTOS_TimeOutValue or 0 in case of no time-out."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMessageQueueGet(
        mq_id: osMessageQueueId_t,
        msg_ptr: *mut cty::c_void,
        msg_prio: *mut u8,
        timeout: u32,
    ) -> osStatus_t;
}
extern "C" {
    #[doc = " Get maximum number of messages in a Message Queue."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\return maximum number of messages."]
    pub fn osMessageQueueGetCapacity(mq_id: osMessageQueueId_t) -> u32;
}
extern "C" {
    #[doc = " Get maximum message size in a Memory Pool."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\return maximum message size in bytes."]
    pub fn osMessageQueueGetMsgSize(mq_id: osMessageQueueId_t) -> u32;
}
extern "C" {
    #[doc = " Get number of queued messages in a Message Queue."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\return number of queued messages."]
    pub fn osMessageQueueGetCount(mq_id: osMessageQueueId_t) -> u32;
}
extern "C" {
    #[doc = " Get number of available slots for messages in a Message Queue."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\return number of available slots for messages."]
    pub fn osMessageQueueGetSpace(mq_id: osMessageQueueId_t) -> u32;
}
extern "C" {
    #[doc = " Reset a Message Queue to initial empty state."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMessageQueueReset(mq_id: osMessageQueueId_t) -> osStatus_t;
}
extern "C" {
    #[doc = " Delete a Message Queue object."]
    #[doc = " \\param[in]     mq_id         message queue ID obtained by \\ref osMessageQueueNew."]
    #[doc = " \\return status code that indicates the execution status of the function."]
    pub fn osMessageQueueDelete(mq_id: osMessageQueueId_t) -> osStatus_t;
}
