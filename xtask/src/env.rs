use std::{
    env,
    path::{Path, PathBuf},
};

pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

pub fn cross_target() -> Option<String> {
    std::env::var("CROSS_TARGET").ok()
}

pub fn target_dir() -> PathBuf {
    let mut result = project_root().join("target");

    if let Some(x) = cross_target() {
        result.push(x);
    }

    result
}

pub fn target_release_bin(name: &str) -> PathBuf {
    target_dir().join("release").join({
        #[cfg(target_family = "windows")]
        {
            format!("{name}.exe")
        }

        #[cfg(target_family = "unix")]
        name
    })
}

pub fn target_release_dylib(name: &str) -> PathBuf {
    target_dir().join("release").join({
        cfg_if::cfg_if! {
            if #[cfg(target_family = "windows")] {
                format!("{name}.dll")
            } else if #[cfg(target_os = "macos")] {
                format!("lib{name}.dylib")
            } else if #[cfg(target_family = "unix")] {
                format!("lib{name}.so")
            } else {
                std::compile_error!("unsupported platform");
            }
        }
    })
}

pub fn dist_dir() -> PathBuf {
    target_dir().join("dist")
}

pub fn dist_bin_dir() -> PathBuf {
    dist_dir().join("bin")
}

pub fn dist_plugins_dir() -> PathBuf {
    dist_dir().join("plugins")
}

pub fn dist_plugin(name: &str) -> PathBuf {
    cfg_if::cfg_if! {
        if #[cfg(target_family = "windows")] {
            dist_plugins_dir().join(&format!("{name}.dll"))
        } else if #[cfg(target_family = "unix")] {
            dist_plugins_dir().join(&format!("{name}.so"))
        } else {
            std::compile_error!("unsupported platform");
        }
    }
}

pub fn dist_bin(name: &str) -> PathBuf {
    cfg_if::cfg_if! {
        if #[cfg(target_family = "windows")] {
            dist_bin_dir().join(&format!("{name}.exe"))
        } else if #[cfg(target_family = "unix")] {
            dist_bin_dir().join(name)
        } else {
            std::compile_error!("unsupported platform");
        }
    }
}