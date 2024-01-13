use airlog_sdk::plugin::{CompatInfo, PluginCompatInfoFn, PluginInitFn};
use anyhow::anyhow;
use libloading::{Library, Symbol};
use std::path::Path;

pub fn open_plugin(name: &str) -> anyhow::Result<()> {
    open_plugin_at(airlog_sdk::env::plugin(name))
}

pub fn open_plugin_at<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let library = unsafe { Library::new(path.as_ref()) }?;

    let compat_info: Symbol<PluginCompatInfoFn> =
        unsafe { library.get(b"airlogd_plugin_compat_info")? };
    if compat_info() != CompatInfo::new() {
        return Err(anyhow!("incompatiable plugin"));
    }

    let plugin_init: Symbol<PluginInitFn> = unsafe { library.get(b"airlogd_plugin_init")? };
    plugin_init(Box::new(super::plugin_api::PluginApi::new()));

    std::mem::forget(library);

    Ok(())
}
