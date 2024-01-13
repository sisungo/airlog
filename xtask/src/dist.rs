use crate::{env, DynError};
use std::{fs, process::Command};

pub fn main() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&env::dist_dir());
    fs::create_dir_all(&env::dist_dir())?;
    fs::create_dir_all(&env::dist_bin_dir())?;
    fs::create_dir_all(&env::dist_plugins_dir())?;

    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(env::project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    dist_binary()?;
    dist_plugins()?;

    Ok(())
}

fn dist_binary() -> Result<(), DynError> {
    send_binary("airlogd")?;
    send_binary("airlog")?;

    Ok(())
}

fn dist_plugins() -> Result<(), DynError> {
    send_plugin("debugger")?;
    send_plugin("syslog_frontend")?;
    send_plugin("textfile_backend")?;

    Ok(())
}

fn send_plugin(name: &str) -> Result<(), DynError> {
    let path = env::target_release_dylib(name);
    fs::copy(&path, env::dist_plugin(name))?;
    Ok(())
}

fn send_binary(name: &str) -> Result<(), DynError> {
    let path = env::target_release_bin(name);
    fs::copy(&path, env::dist_bin(name))?;
    Ok(())
}
