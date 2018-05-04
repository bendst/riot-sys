extern crate bindgen;
extern crate git2;
extern crate config;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::path::PathBuf;

#[derive(Deserialize)]
struct AllBoards {
    common: Vec<String>,
}

#[derive(Deserialize)]
struct BoardData {
    model: String,
    defines: Vec<String>,
    includes: Vec<String>,
}


fn update_readme() {
    use std::process::Command;
    use std::fs::File;
    use std::io::Write;

    let output = Command::new("cargo").arg("readme").output().unwrap();
    
    let readme = String::from_utf8_lossy(&output.stdout);

    File::create("README.md").as_mut().map(|file| {
        file.write(readme.as_bytes()).unwrap();
    }).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed:src/lib.rs");
    println!("cargo:rerun-if-changed:config/board.toml");
    update_readme();

    let mut settings = config::Config::new();
    settings
        .merge(config::File::with_name("config/board"))
        .unwrap();

    let features = env::vars()
        .filter_map(|(key, _value)| {
            let feature = key.trim_left_matches("CARGO_FEATURE_");
            if key.contains("CARGO_FEATURE_") {
                Some(feature.to_lowercase().replace('_', "-"))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let common = settings.get::<AllBoards>("all").expect("No common field");

    // TODO abort better
    assert!(features.len() == 1, "Must at least configure one board.");

    let board_path = &format!("board.{}", features[0]);
    let board = settings.get::<BoardData>(board_path).expect(
        "No such board",
    );

    let mut clang_args = Vec::with_capacity(32);
    clang_args.extend(common.common);
    clang_args.push(board.model);
    clang_args.extend(board.defines);
    clang_args.extend(board.includes);

    let repo = git2::Repository::open(env::current_dir().unwrap()).unwrap();
    let mut submodules = repo.submodules().unwrap();

    for submodules in &mut submodules {
        submodules.update(true, None).unwrap();
    }

    let bindings = bindgen::builder()
        .use_core()
        //.derive_copy(false)
        .ctypes_prefix("cty")
        .clang_args(clang_args)
        .generate_comments(true)
        .whitelist_function("thread_.*")
        .whitelist_var("THREAD_.*")
        .whitelist_var("STATUS_.*")
        .whitelist_function("mutex_.*")
        .whitelist_function("_mutex_.*")
        .whitelist_var("MUTEX_.*")
        .whitelist_type("MUTEX_.*")
        .whitelist_var("sched_active_pid")
        .whitelist_var("KERNEL_PID_UNDEF")
        .whitelist_function("timex_.*")
        .whitelist_function("xtimer_.*")
        .whitelist_function("print.*")
        .header("RIOT/sys/include/timex.h")
        .header("RIOT/sys/include/xtimer.h")
        .header("RIOT/core/include/thread.h")
        .header("RIOT/core/include/sched.h")
        .header("RIOT/core/include/mutex.h")
        .header("RIOT/sys/include/fmt.h")
        .rust_target(bindgen::RustTarget::Nightly)
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
