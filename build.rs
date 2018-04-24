extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

fn main() {

    let mut board_map = HashMap::new();
    board_map.insert(
        "sam0_common",
        vec![
            "-DCPU_MODEL_SAMR21G18A",
            "-DDONT_USE_CMSIS_INIT",
            "-IRIOT/cpu/cortexm_common/include",
            "-IRIOT/cpu/cortexm_common/include/vendor",
            "-IRIOT/cpu/sam0_common/include/",
            "-IRIOT/cpu/sam0_common/include/vendor",
            "-IRIOT/cpu/sam0_common/include/vendor/samr21/include",
            "-IRIOT/cpu/sam0_common/include/vendor/samr21/include",
            "-IRIOT/cpu/sam0_common/include/vendor/samrl21/include",
            "-IRIOT/cpu/sam0_common/include/vendor/samrd21/include",
        ],
    );

    let board = "sam0_common";


    let bindings = bindgen::builder()
        .use_core()
        .derive_copy(false)
        .ctypes_prefix("cty")
        .clang_args(&board_map[board])
        .generate_comments(true)
        .whitelist_function("thread_.*")
        .whitelist_var("THREAD_.*")
        .whitelist_var("STATUS_.*")
        .whitelist_function("mutex_.*")
        .whitelist_function("_mutex_.*")
        .whitelist_var("MUTEX_.*")
        .whitelist_type("MUTEX_.*")
        .whitelisted_var("sched_active_pid")
        .whitelisted_var("KERNEL_PID_UNDEF")
        .header("RIOT/core/include/thread.h")
        .header("RIOT/core/include/sched.h")
        .header("RIOT/core/include/mutex.h")
        .rust_target(bindgen::RustTarget::Nightly)
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
