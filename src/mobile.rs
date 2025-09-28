use crate::model::Results;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_value, Value};

use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[derive(Serialize)]
struct SetItemPayload<'a> {
    key: &'a str,
    value: &'a str,
    group: &'a str,
}

#[derive(Serialize)]
struct GetItemsPayload<'a> {
    key: &'a str,
    group: &'a str,
}

#[derive(Serialize)]
struct SetRegisterPayload{
    widgets: Vec<String>
}

#[derive(Serialize)]
struct ReloadTimeLinesPayload<'a> {
    of_kind: &'a str
}

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "git.fwy13.widget";

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Widget<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "WidgetBridePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_widget)?;
    Ok(Widget(handle))
}

pub struct Widget<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Widget<R> {
    pub fn set_items(&self, key: &str, value: &str, group: &str) -> crate::Result<Results<bool>> {
        let value : Value = self.0
            .run_mobile_plugin("setItems", SetItemPayload { key, value, group })?;
        let parsed: Results<bool> = from_value(value)?;
        Ok(parsed)
    }

    pub fn get_items(&self, key: &str, group: &str) -> crate::Result<Results<String>> {
        let value: Value = self
            .0
            .run_mobile_plugin("getItems", GetItemsPayload { key, group })?;

        let parsed: Results<String> = from_value(value)?;
        Ok(parsed)
    }
    pub fn set_register_widget(&self, widgets: Vec<String>) -> crate::Result<Results<bool>> {
        let value: Value = self
            .0
            .run_mobile_plugin("setRegisterWidget", SetRegisterPayload { widgets })?;
        let parsed: Results<bool> = from_value(value)?;
        Ok(parsed)
    }
    pub fn reload_all_time_lines(&self) -> crate::Result<Results<bool>> {
        let value: Value = self
            .0
            .run_mobile_plugin("reloadAllTimelines", ())?;
        let parsed: Results<bool> = from_value(value)?;
        Ok(parsed)
    }
    pub fn reload_time_lines(&self, of_kind: &str) -> crate::Result<Results<bool>> {
        let value: Value = self
            .0
            .run_mobile_plugin("reloadTimelines", ReloadTimeLinesPayload {
                of_kind
            })?;
        let parsed: Results<bool> = from_value(value)?;
        Ok(parsed)
    }
    pub fn request_widget(&self) -> crate::Result<Results<bool>> {
        let value : Value = self
            .0
            .run_mobile_plugin("requestWidget", ())?;
        let parsed: Results<bool> = from_value(value)?;
        Ok(parsed)
    }
}
