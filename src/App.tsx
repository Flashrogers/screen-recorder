import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import "./App.css";

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [preset, setPreset] = useState("Docs");

  const toggleRecording = async () => {
  if (!isRecording) {
    await invoke("start_recording", { preset });
    setIsRecording(true);
  } else {
    await invoke("stop_recording");
    setIsRecording(false);
  }
};
  return (
    <div className="app">
      <div className="card">
        <h1>🎥 Screen Recorder</h1>

        <div className="preset">
          <label>Preset</label>
          <select
            value={preset}
            onChange={(e) => setPreset(e.target.value)}
          >
            <option value="Docs">Support Docs</option>
            <option value="Demo">Product Demo</option>
            <option value="Tutorial">Tutorial Video</option>
          </select>
        </div>

        <div className="status">
          {isRecording ? (
            <span className="recording">● Recording...</span>
          ) : (
            <span className="idle">● Idle</span>
          )}
        </div>

        <button
          className={isRecording ? "stop" : "start"}
          onClick={toggleRecording}
        >
          {isRecording ? "Stop Recording" : "Start Recording"}
        </button>

        <div className="actions">
          <button>📋 Copy</button>
          <button>☁ Upload</button>
          <button>📁 Open Folder</button>
        </div>
      </div>
    </div>
  );
}

export default App;
