//! System bindings for RIOT os generated by [bindgen]
//!
//! [bindgen]: https://github.com/rust-lang-nursery/rust-bindgen.git
//!
//! # Supported bindings
//! - [x] threads
//! - [x] mutex
//! - [x] network
//! - [x] time
//! - [x] formatted output
//!
//! # Supported boards
//! - samr21-xpro
//!
//! # How to add X
//!
//! ## Board
//! To add a another board you must edit the `Cargo.toml` and the `config/board.toml`
//! In the `Cargo.toml`, you must add the new board as an feature. The feature must match the
//! board configuration in the `config/board`.
//!
//! If you discover more common preprocessor configuration you can add them within `[all]`.
//!
//! ## Functionality
//! Modify the `whitelist_[function|var|type]` in the `build.rs` and optionally add a new header
//! for which bindings should be generated.
//! It will likely be the case, that after adding a new header, that some board modification must
//! be extended with more preprocessor configuration within the `config/board.toml`.


#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

extern crate cty;


pub mod ffi {
    use cty;

    pub use cty::*;

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

    pub const SOCK_IPV6_EP_ANY: _sock_tl_ep = _sock_tl_ep {
        family: AF_INET6 as _,
        addr: _sock_tl_ep__bindgen_ty_1 { ipv6: [0; 16] },
        netif: SOCK_ADDR_ANY_NETIF as _,
        port: 0,
    };

    #[inline(always)]
    pub unsafe fn gnrc_netif_ipv6_group_join(
        netif: *const gnrc_netif_t,
        group: *mut ipv6_addr_t,
    ) -> cty::c_int {
        use core::mem;
        gnrc_netapi_set(
            (*netif).pid,
            netopt_t_NETOPT_IPV6_GROUP,
            0,
            group as *mut _,
            mem::size_of::<ipv6_addr_t>(),
        )
    }

    #[inline(always)]
    pub unsafe fn gnrc_netif_ipv6_group_leave(
        netif: *const gnrc_netif_t,
        group: *mut ipv6_addr_t,
    ) -> cty::c_int {
        use core::mem;
        gnrc_netapi_set(
            (*netif).pid,
            netopt_t_NETOPT_IPV6_GROUP_LEAVE,
            0,
            group as *mut _,
            mem::size_of::<ipv6_addr_t>(),
        )
    }

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
