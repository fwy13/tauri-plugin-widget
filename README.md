# 📦 Tauri Plugin widget

A Tauri plugin to interact with App Widgets (Android). Allows your Tauri app to shared preferences (Android), and update timeline widgets.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/tauri-plugin-widget.svg
[crates-url]: https://crates.io/crates/tauri-plugin-widget
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

> [!NOTE]  
> Thanks for the idea from https://github.com/kisimediaDE/capacitor-widget-bridge

## 🎬 Demo

<div style="display: flex; gap: 20px; align-items: center; justify-content: center;">
  <img style="border-radius: 40px; border: 1px solid black;" src="https://raw.githubusercontent.com/fwy13/tauri-plugin-widget/refs/heads/main/demo.gif" alt="Android Example Demo" width="50%" />
</div>

## 🚀 Install

- Rust

```toml
[dependencies]
tauri-plugin-widget = "0.1.2"
```

- Javascript (npm, pnpm yarn, ...)

```bash
npm install tauri-plugin-widget-api
```

## 📱 Setup

- First, in `src-tauri/lib.rs`

```rust
pub fn run() {
    tauri::Builder::default()
        // Register here.
        .plugin(tauri_plugin_widget::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- Next, in `ui (react, svelte, vue, ...)`

```javascript
import {
  setItems,
  getItems,
  setRegisterWidget,
  reloadTimelines,
  requestWidget,
} from "tauri-plugin-widget-api";
```

> [!IMPORTANT]  
> You always set register the widget classes in JS code
> await setRegisterWidget(["com.example.widgetbrideexample.MyWidget"]);

## 📖 Api

## `setItems(key: string, value: string, group: string): Promise<DataResult<boolean>>`

Stores a `key-value` pair in the widget storage under a specific group.

- **key**: The key under which the value will be stored.
- **value**: The string value to store.
- **group**: The namespace or group to organize stored items.

**Returns**:  
`Promise<DataResult<boolean>>` – `true` if the item was stored successfully.

---

## `getItems(key: string, group: string): Promise<DataResult<any>>`

Retrieves a value from the widget storage based on `key` and `group`.

- **key**: The key to retrieve.
- **group**: The namespace containing the item.

**Returns**:  
`Promise<DataResult<any>>` – The stored value (any type) or `null` if not found.

---

## `reloadAllTimelines(): Promise<DataResult<boolean>>`

Requests the system to reload **all widget timelines** (refreshes every registered widget).

**Returns**:  
`Promise<DataResult<boolean>>` – `true` if the reload was successful.

---

## `reloadTimelines(ofKind: string): Promise<DataResult<boolean>>`

Requests the system to reload timelines of a **specific kind** of widget.

- **ofKind**: The widget/timeline type to reload.

**Returns**:  
`Promise<DataResult<boolean>>` – `true` if the reload was successful.

---

## `setRegisterWidget(widgets: string[]): Promise<DataResult<boolean>>`

Registers a list of widgets that the application will manage.

- **widgets**: An array of widget identifiers or names.

**Returns**:  
`Promise<DataResult<boolean>>` – `true` if registration was successful.

---

## `requestWidget(): Promise<DataResult<boolean>>`

Requests the system to provide or display the registered widget.  
(Useful for triggering the widget add flow on the home screen).

**Returns**:  
`Promise<DataResult<boolean>>` – `true` if the request was successful.

## 📱 Platform

- This plugin only support android.
- You can contribute an IOS version to it.

## 🪪 License

MIT
