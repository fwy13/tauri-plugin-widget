use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod model;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Widget;
#[cfg(mobile)]
use mobile::Widget;

pub trait WidgetExt<R: Runtime> {
    fn widget(&self) -> &Widget<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WidgetExt<R> for T {
    fn widget(&self) -> &Widget<R> {
        self.state::<Widget<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("widget")
        .invoke_handler(tauri::generate_handler![
            commands::set_register_widget,
            commands::set_items,
            commands::get_items,
            commands::reload_all_time_lines,
            commands::reload_time_lines,
            commands::request_widget
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let widget = mobile::init(app, api)?;
            #[cfg(desktop)]
            let widget = desktop::init(app, api)?;
            app.manage(widget);
            Ok(())
        })
        .build()
}
