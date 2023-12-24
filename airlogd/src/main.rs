#[cfg_attr(feature = "splited", path = "plugin_loader_splited.rs")]
pub mod plugin_loader;

pub mod plugin_api;

#[derive(clap::Parser)]
struct Cmdline {}

#[tokio::main]
async fn main() {
    let _cmdline = <Cmdline as clap::Parser>::parse();
}
