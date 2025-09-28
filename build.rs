const COMMANDS: &[&str] = &["set_items", "get_items", "set_register_widget", "reload_all_time_lines", "reload_time_lines", "request_widget"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
