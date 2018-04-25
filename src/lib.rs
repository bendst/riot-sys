#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

extern crate cty;

pub mod ffi {
    use cty;

    /// @brief Returns the process ID of the currently running thread
    ///
    /// @return          obviously you are not a golfer.
    ///
    #[inline(always)]
    pub unsafe fn thread_getpid() -> kernel_pid_t {
        use core::ptr;
        ptr::read_volatile(&sched_active_pid)
    }

    /// @brief Initializes a mutex object.
    ///
    /// @details For initialization of variables use MUTEX_INIT instead.
    /// Only use the function call for dynamically allocated mutexes.
    /// @param[out] mutex    pre-allocated mutex structure, must not be NULL.
    #[inline(always)]
    pub unsafe fn mutex_init(mutex: *mut mutex_t) {
        use core::ptr;
        (*mutex).queue.next = ptr::null_mut();
    }

    /// @brief Locks a mutex, blocking.
    ///
    /// @param[in] mutex Mutex object to lock. Has to be initialized first.
    /// Must not be NULL.
    #[inline(always)]
    pub unsafe fn mutex_lock(mutex: *mut mutex_t) {
        _mutex_lock(mutex, 1);
    }

    /// @brief Tries to get a mutex, non-blocking.
    ///
    /// @param[in] mutex Mutex object to lock. Has to be initialized first.
    /// Must not be NULL.
    ///
    /// @return 1 if mutex was unlocked, now it is locked.
    /// @return 0 if the mutex was locked.
    #[inline(always)]
    pub unsafe fn mutex_trylock(mutex: *mut mutex_t) -> cty::c_int {
        _mutex_lock(mutex, 0)
    }

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
