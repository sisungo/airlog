#[cfg_attr(feature = "splited", path = "plugin_loader_splited.rs")]
pub mod plugin_loader;

pub mod pipeline;
pub mod plugin_api;

#[derive(clap::Parser)]
struct Cmdline {
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    if airlog_sdk::env::airlog_dist_dir().is_none() {
        std::process::exit(12);
    }

    let cmdline = <Cmdline as clap::Parser>::parse();

    if cmdline.debug {
        if let Err(_) = plugin_loader::open_plugin("debugger") {
            std::process::exit(13);
        }
    }
}
