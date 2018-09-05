extern crate bindgen;
extern crate config;
extern crate git2;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::path::{Path, PathBuf};

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
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    let output = Command::new("cargo").arg("readme").output().unwrap();

    let readme = String::from_utf8_lossy(&output.stdout);

    File::create("README.md")
        .as_mut()
        .map(|file| {
            file.write(readme.as_bytes())
                .expect("failed to write to README.md");
        }).expect("failed to create README.md");
}

fn header_exists<A: AsRef<Path>>(path: A) -> Option<PathBuf> {
    use std::fs;
    fs::metadata(&path).ok().and_then(|meta| {
        if meta.is_file() {
            Some(path.as_ref().into())
        } else {
            None
        }
    })
}

fn clang_version() -> String {
    use std::process::Command;

    let mut cmd = Command::new("clang");
    cmd.arg("--version");
    let output = match cmd.output() {
        Ok(output) => output,
        Err(_) => return String::new(),
    };
    String::from_utf8(output.stdout).unwrap_or_else(|_| String::new())
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=config/board.toml");
    update_readme();

    let mut settings = config::Config::new();
    settings
        .merge(config::File::with_name("config/board"))
        .expect("unable to merge with config file.");

    let features = env::vars()
        .filter_map(|(key, _value)| {
            let feature = key.trim_left_matches("CARGO_FEATURE_");
            if key.contains("CARGO_FEATURE_") {
                Some(feature.to_lowercase().replace('_', "-"))
            } else {
                None
            }
        }).collect::<Vec<_>>();

    let common = settings.get::<AllBoards>("all").expect("No common field");

    // TODO abort better
    assert!(features.len() == 1, "Must at least configure one board.");

    let board_path = &format!("board.{}", features[0]);
    let board = settings
        .get::<BoardData>(board_path)
        .expect("No such board");

    let mut clang_args = Vec::with_capacity(32);
    match clang_version().as_str() {
        v if v.contains("6.") => (),
        _ => clang_args.push("--std=c11".to_owned()),
    }
    clang_args.extend(common.common);
    clang_args.push(board.model);
    clang_args.extend(board.defines);
    clang_args.extend(board.includes);

    let repo =
        git2::Repository::open(env::current_dir().expect("Could not retrieve current directory."))
            .expect("Failed to open current directory.");
    let mut submodules = repo.submodules().expect("No submodules.");

    for submodules in &mut submodules {
        submodules
            .update(true, None)
            .expect("Failed to update submodules");
    }
    let errno_path = env::var("CARGO_CFG_TARGET_ARCH")
        .map(|arch| match arch.as_str() {
            "arm" => {
                let path = "/usr/arm-none-eabi/include/errno.h";
                header_exists(path)
                    .or_else(|| env::var("SDDS_ERRNO_PATH").ok().map(From::from))
                    .expect(
                        "No errno header found. You can set it manually by using SDDS_ERRNO_PATH",
                    )
            }
            _ => PathBuf::from("/usr/include/errno.h"),
        }).or_else(|_| env::var("SDDS_ERRNO_PATH").map(From::from))
        .expect("Failed to determine errno header");

    let bindings = bindgen::builder()
        .use_core()
        //.derive_copy(false)
        .ctypes_prefix("cty")
        .clang_args(clang_args)
        .generate_comments(true)
        .whitelist_type("MUTEX_.*")
        .whitelist_type("SOCK_.*")
        .whitelist_type("sock_udp.*")
        .whitelist_type("AF_.*")
        .whitelist_type("ipv6_addr_t")
        .whitelist_type("gnrc_netif_t")
        .whitelist_type("eui64_t")
        .whitelist_type("netopt_t")
        .whitelist_var("E.*")
        .whitelist_var("AF_.*")
        .whitelist_var("THREAD_.*")
        .whitelist_var("STATUS_.*")
        .whitelist_var("MUTEX_.*")
        .whitelist_var("SOCK_.*")
        .whitelist_var("sched_active_pid")
        .whitelist_var("KERNEL_PID_UNDEF")
        .whitelist_function("thread_.*")
        .whitelist_function("mutex_.*")
        .whitelist_function("_mutex_.*")
        .whitelist_function("timex_.*")
        .whitelist_function("xtimer_.*")
        .whitelist_function("print.*")
        .whitelist_function("sock_.*")
        .whitelist_function("gnrc_netapi_set")
        .whitelist_function("gnrc_netif_iter")
        .whitelist_function("netdev_eth_get")
        //.header("RIOT/cpu/atmega_common/avr-libc-extra/errno.h")
        .header(errno_path.to_str().expect("Invalid str"))
        .header("RIOT/sys/include/timex.h")
        .header("RIOT/sys/include/xtimer.h")
        .header("RIOT/core/include/thread.h")
        .header("RIOT/core/include/sched.h")
        .header("RIOT/core/include/mutex.h")
        .header("RIOT/sys/include/fmt.h")
        .header("RIOT/sys/include/net/sock/udp.h")
        .header("RIOT/sys/include/net/sock.h")
        .header("RIOT/sys/include/net/gnrc/netapi.h")
        .header("RIOT/sys/include/net/gnrc/netif.h")
        .header("RIOT/sys/include/net/netopt.h")
        .header("RIOT/sys/include/net/af.h")
        .header("RIOT/sys/include/net/ipv6/addr.h")
        .header("RIOT/sys/include/net/ethernet.h")
        .header("RIOT/drivers/include/net/netdev/eth.h")
        .rust_target(bindgen::RustTarget::Nightly)
        .rustfmt_bindings(false)
        .generate()
        .expect("Failed to generate bindings");

    let out_path =
        PathBuf::from(env::var("OUT_DIR").expect("failed to create output directory path"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
