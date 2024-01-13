use airlog_sdk::plugin::{CompatInfo, PluginTable};

#[no_mangle]
pub extern "C" fn airlogd_plugin_compat_info() -> CompatInfo {
    CompatInfo::new()
}

#[no_mangle]
pub extern "Rust" fn airlogd_plugin_init(table: Box<dyn PluginTable>) {
    table.async_runtime().spawn(async move {
        println!("Hello from plugin `debug.so` using `tokio`");
    });
}
