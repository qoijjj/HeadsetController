const { invoke } = window.__TAURI__.tauri;

let configFormEl;
let headsetPresenceEl;

async function isHeadsetFound() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const isHeadsetFound = await invoke("is_headset_found");
  console.log(isHeadsetFound);
  headsetPresenceEl.textContent =
    isHeadsetFound ? "Supported headset found.": "No supported headset found.";

  if (isHeadsetFound) {
    headsetPresenceEl.style.display = "block"
  }
}

window.addEventListener("DOMContentLoaded", () => {
  headsetPresenceEl = document.querySelector("#headset-presence");
  configFormEl = document.querySelector("#config-form");
  setInterval(async function(){ 
    await isHeadsetFound();   
  }, 1000);
});
