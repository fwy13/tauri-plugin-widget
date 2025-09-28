use crate::Result;
use crate::WidgetExt;
use tauri::{command, AppHandle, Runtime};

use crate::model::Results;

#[command]
pub(crate) async fn set_items<R: Runtime>(
    app: AppHandle<R>,
    key: &str,
    value: &str,
    group: &str,
) -> Result<Results<bool>> {
    let value: Results<bool> = app.widget().set_items(key, value, group)?;
    Ok(value)
}

#[command]
pub(crate) async fn get_items<R: Runtime>(
    app: AppHandle<R>,
    key: &str,
    group: &str,
) -> Result<Results<String>> {
    let value: Results<String> = app.widget().get_items(key, group)?;
    Ok(value)
}

#[command]
pub(crate) async fn set_register_widget<R: Runtime>(
    app: AppHandle<R>,
    widgets: Vec<String>,
) -> Result<Results<bool>> {
    let value: Results<bool> = app.widget().set_register_widget(widgets)?;
    Ok(value)
}

#[tauri::command]
pub(crate) async fn reload_all_time_lines<R: Runtime>(app: AppHandle<R>) -> Result<Results<bool>> {
    let value: Results<bool> = app.widget().reload_all_time_lines()?;
    Ok(value)
}

#[tauri::command]
pub(crate) async fn reload_time_lines<R: Runtime>(app: AppHandle<R>, of_kind: &str) -> Result<Results<bool>> {
    let value: Results<bool> = app.widget().reload_time_lines(of_kind)?;
    Ok(value)
}

#[tauri::command]
pub(crate) async  fn request_widget<R: Runtime>(app: AppHandle<R>) -> Result<Results<bool>> {
    let value: Results<bool> = app.widget().request_widget()?;
    Ok(value)
}
