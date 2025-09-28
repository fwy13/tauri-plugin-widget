import { useEffect, useState } from "react";
import {
  setItems,
  getItems,
  requestWidget,
  setRegisterWidget,
  reloadAllTimelines,
} from "tauri-plugin-widget-api";
import "./App.css";

function App() {
  const [inputValue, setInputValue] = useState("");
  const [result, setResult] = useState("");

  async function setup() {
    await setRegisterWidget(["com.fwy13.tauri_plugin_widget_example.Widget"]);
  }

  async function handleSetItem() {
    const res = await setItems(
      "widget_text",
      inputValue,
      "com.fwy13.tauri_plugin_widget_example.Widget"
    );
    await reloadAllTimelines();
    setResult(`Set Item: ${res.results}`);
  }

  async function handleGetItem() {
    const res = await getItems(
      "widget_text",
      "com.fwy13.tauri_plugin_widget_example.Widget"
    );
    setResult(`Get Item: ${JSON.stringify(res.results)}`);
  }

  async function handleRequestWidget() {
    const res = await requestWidget();
    setResult(`Request Widget: ${res.results}`);
  }

  useEffect(() => {
    setup();
  }, []);

  return (
    <main className="container">
      <div className="widget-box">
        <h1>Widget Plugin Demo</h1>
        <input
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.currentTarget.value)}
          placeholder="Enter some text..."
          className="input"
        />

        <div className="button-row">
          <button className="btn" onClick={handleSetItem}>
            Set Item
          </button>
          <button className="btn" onClick={handleGetItem}>
            Get Item
          </button>
          <button className="btn" onClick={handleRequestWidget}>
            Request Widget
          </button>
        </div>

        <p className="result">{result}</p>
      </div>
    </main>
  );
}

export default App;
