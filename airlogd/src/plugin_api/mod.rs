use airlog_sdk::plugin::PluginTable;

pub struct PluginApi {}
impl PluginApi {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait::async_trait]
impl PluginTable for PluginApi {
    fn async_runtime(&self) -> tokio::runtime::Handle {
        tokio::runtime::Handle::current()
    }
}
