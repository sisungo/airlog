use airlog_sdk::plugin::{CompatInfo, PluginInfo, PluginTable};

#[no_mangle]
pub extern "C" fn airlogd_plugin_debugger_compat_info() -> CompatInfo {
    CompatInfo::new()
}

#[no_mangle]
pub extern "Rust" fn airlogd_plugin_debugger_info() -> PluginInfo {
    PluginInfo {}
}

#[no_mangle]
pub extern "Rust" fn airlogd_plugin_debugger_init(table: Box<dyn PluginTable>) {
    table.async_runtime().spawn(async move {});
}