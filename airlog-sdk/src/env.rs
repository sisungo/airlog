use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub fn airlog_dist_dir() -> Option<&'static Path> {
    static AIRLOG_DIST_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();

    AIRLOG_DIST_DIR
        .get_or_init(airlog_dist_dir_uncached)
        .as_deref()
}

pub fn plugin(name: &str) -> PathBuf {
    airlog_dist_dir()
        .expect("check `airlog_dist_dir` first")
        .join(format!("plugins/{name}.so"))
}

fn airlog_dist_dir_uncached() -> Option<PathBuf> {
    let from_env = std::env::var_os("AIRUP_DIST_DIR").map(PathBuf::from);
    if let Some(x) = from_env {
        Some(x)
    } else {
        std::env::current_exe()
            .ok()
            .and_then(|x| std::fs::canonicalize(x).ok())
            .and_then(|x| x.ancestors().nth(2).map(PathBuf::from))
    }
}
