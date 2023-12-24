use airlog_sdk::plugin::{PluginCompatInfoFn, PluginInfo, CompatInfo, PluginInfoFn, PluginInitFn};
use anyhow::anyhow;
use libloading::{Library, Symbol};
use std::path::Path;

pub fn open_plugin<P: AsRef<Path>>(path: P) -> anyhow::Result<PluginInfo> {
    let name = path
        .as_ref()
        .file_stem()
        .ok_or_else(|| anyhow!("Is a directory"))?;

    let library = unsafe { Library::new(path.as_ref()) }?;

    let compat_info_name = format!("airlogd_plugin_{}_compat_info", name.to_string_lossy());
    let compat_info: Symbol<PluginCompatInfoFn> =
        unsafe { library.get(compat_info_name.as_bytes())? };
    if compat_info() != CompatInfo::new() {
        return Err(anyhow!("incompatiable plugin"));
    }

    let plugin_info_name = format!("airlogd_plugin_{}_info", name.to_string_lossy());
    let plugin_info: Symbol<PluginInfoFn> = unsafe { library.get(plugin_info_name.as_bytes())? };
    let info = plugin_info();

    let plugin_init_name = format!("airlogd_plugin_{}_init", name.to_string_lossy());
    let plugin_init: Symbol<PluginInitFn> = unsafe { library.get(plugin_init_name.as_bytes())? };
    plugin_init(Box::new(super::plugin_api::PluginApi::new()));

    std::mem::forget(library);

    Ok(info)
}