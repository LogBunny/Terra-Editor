"use client"
import init, { run_bevy_app } from "../pkg/terrain_editor";

export default function Home() {
  const runBevyApp = async () => {
    await init();
    run_bevy_app();
  };
  return (
    <div className="App">
      <canvas id="mygame-canvas" width={128} height={128} className="w-[512px!important] h-[512px!important]"></canvas>
    
      <button onClick={runBevyApp}>Run Bevy App</button>
    </div>
  );
}
