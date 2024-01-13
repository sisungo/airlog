pub mod env;
pub mod plugin;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
