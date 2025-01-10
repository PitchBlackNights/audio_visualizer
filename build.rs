use std::{
    env, fs,
    path::{Path, PathBuf},
};
use vergen_git2::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    === ENV VARIABLES: ===
    VERGEN_BUILD_TIMESTAMP
    VERGEN_CARGO_TARGET_TRIPLE
    VERGEN_GIT_BRANCH
    VERGEN_GIT_COMMIT_TIMESTAMP
    VERGEN_GIT_SHA
    VERGEN_RUSTC_CHANNEL
    VERGEN_RUSTC_COMMIT_DATE
    VERGEN_RUSTC_COMMIT_HASH
    VERGEN_RUSTC_SEMVER

    VERGEN_SYSINFO_OS_VERSION
    VERGEN_SYSINFO_TOTAL_MEMORY
    VERGEN_SYSINFO_CPU_CORE_COUNT
    VERGEN_SYSINFO_CPU_BRAND
    VERGEN_SYSINFO_CPU_FREQUENCY
    */

    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let cargo = CargoBuilder::default().target_triple(true).build()?;
    let git2 = Git2Builder::default()
        .branch(true)
        .commit_timestamp(true)
        .sha(false)
        .build()?;
    let rustc = RustcBuilder::default()
        .channel(true)
        .commit_date(true)
        .commit_hash(true)
        .semver(true)
        .build()?;
    // let si = SysinfoBuilder::default()
    //     .os_version(true)
    //     .memory(true)
    //     .cpu_core_count(true)
    //     .cpu_brand(true)
    //     .cpu_frequency(true)
    //     .build()?;

    Emitter::default()
        // .idempotent()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&git2)?
        .add_instructions(&rustc)?
        // .add_instructions(&si)?
        .emit_and_set()?;

    let env_list: Vec<&str> = vec![
        "CARGO_PKG_VERSION",
        "VERGEN_BUILD_TIMESTAMP",
        "VERGEN_CARGO_TARGET_TRIPLE",
        "VERGEN_GIT_SHA",
        "VERGEN_GIT_COMMIT_TIMESTAMP",
        "VERGEN_GIT_BRANCH",
        "VERGEN_RUSTC_CHANNEL",
        "VERGEN_RUSTC_COMMIT_DATE",
        "VERGEN_RUSTC_COMMIT_HASH",
        "VERGEN_RUSTC_SEMVER",
        // "VERGEN_SYSINFO_OS_VERSION",
        // "VERGEN_SYSINFO_TOTAL_MEMORY",
        // "VERGEN_SYSINFO_CPU_CORE_COUNT",
        // "VERGEN_SYSINFO_CPU_BRAND",
        // "VERGEN_SYSINFO_CPU_FREQUENCY",
    ];
    let out_dir: String = env::var("OUT_DIR").unwrap();
    let dest_path: PathBuf = Path::new(&out_dir).join("env_vars.rs");
    let mut env_vars: String = String::from("pub const ENV_VARS: &[(&str, &str)] = &[");

    for environ in env_list {
        let concat_string: String = match env::var(environ) {
            Ok(value) => format!("(\"{}\", \"{}\"),\n", environ, value),
            Err(_) => format!(""),
        };

        env_vars.push_str(&concat_string);
    }

    env_vars.push_str("\n];\n");
    fs::write(dest_path, env_vars).unwrap();

    Ok(())
}
