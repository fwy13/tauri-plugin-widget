use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::model::Results;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Widget<R>> {
    Ok(Widget(app.clone()))
}

pub struct Widget<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Widget<R> {
    pub fn set_items(&self, _: &str, _: &str, _: &str) -> crate::Result<Results<bool>> {
        Ok(Results {
            results: Some(false),
        })
    }

    pub fn get_items(&self, _: &str, _: &str) -> crate::Result<Results<String>> {
        Ok(Results {
            results: Some(String::from("")),
        })
    }
    pub fn set_register_widget(&self, _: Vec<String>) -> crate::Result<Results<bool>> {
        Ok(Results {
            results: Some(false),
        })
    }
    pub fn reload_all_time_lines(&self) -> crate::Result<Results<bool>> {
        Ok(Results {
            results: Some(false),
        })
    }
    pub fn reload_time_lines(&self, _: &str) -> crate::Result<Results<bool>> {
        Ok(Results {
            results: Some(false),
        })
    }
    pub fn request_widget(&self) -> crate::Result<Results<bool>> {
        Ok(Results {
            results: Some(false),
        })
    }
}
