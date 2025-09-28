import { invoke } from "@tauri-apps/api/core";

const IDENTIFIER = "plugin:widget";

type DataResult<T> = {
  results: T;
};

export async function setItems(
  key: string,
  value: string,
  group: string
): Promise<DataResult<boolean>> {
  return await invoke(`${IDENTIFIER}|set_items`, {
    key,
    value,
    group,
  });
}

export async function getItems(
  key: string,
  group: string
): Promise<DataResult<any>> {
  return await invoke(`${IDENTIFIER}|get_items`, {
    key,
    group,
  });
}

export async function reloadAllTimelines(): Promise<DataResult<boolean>> {
  return await invoke(`${IDENTIFIER}|reload_all_time_lines`);
}

export async function reloadTimelines(ofKind: string): Promise<DataResult<boolean>> {
  return await invoke(`${IDENTIFIER}|reload_time_lines`, {
    ofKind,
  });
}

export async function setRegisterWidget(widgets: string[]): Promise<DataResult<boolean>> {
  return await invoke(`${IDENTIFIER}|set_register_widget`, {
    widgets,
  });
}

export async function requestWidget(): Promise<DataResult<boolean>> {
  return await invoke(`${IDENTIFIER}|request_widget`);
}
