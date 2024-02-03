const { invoke } = window.__TAURI__.tauri;


async function isHeadsetFound() {
  const headsetPresenceEl = document.querySelector("#headset-presence");
  const isHeadsetFound = await invoke("is_headset_found");
  console.log(isHeadsetFound);
  headsetPresenceEl.textContent =
    isHeadsetFound ? "Supported headset found.": "No supported headset found.";

  if (isHeadsetFound) {
    headsetPresenceEl.style.display = "block"
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  const configFormEl = document.querySelector("#config-form");

  const existingConfig = await invoke("read_config");
  for (const key in existingConfig) {
    document.getElementById(key).value = existingConfig[key];
  }

  const event = new Event('input');
  document.getElementById("sidetone").dispatchEvent(event);
  document.getElementById("preset").dispatchEvent(event);
  
  configFormEl.addEventListener("input", async () => {
      console.log("Form has changed!");
      const formData = new FormData(configFormEl);
      formData.append("sidetone", document.getElementById("sidetone").value);
      formData.append("preset", document.getElementById("preset").value);
      const newConfig = {};
      formData.forEach((value, key) => newConfig[key] = parseInt(value));
      console.log(newConfig);
      await invoke("write_config", { headsetConfig: newConfig });
  });

  setInterval(async function(){ 
    await isHeadsetFound();   
  }, 1000);
});
