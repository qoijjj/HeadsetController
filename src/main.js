const { invoke } = window.__TAURI__.tauri;


async function isHeadsetFound(configFormEl) {
  const headsetPresenceEl = document.querySelector("#headset-presence");
  const isHeadsetFound = await invoke("is_headset_found");
  console.log(isHeadsetFound);
  headsetPresenceEl.textContent =
    isHeadsetFound ? "Supported headset found.": "No supported headset found.";

  if (isHeadsetFound) {
    configFormEl.style.display = "block"
  } else { 
    headsetPresenceEl.style.display = "none"
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  const configFormEl = document.querySelector("#config-form");

  const existingConfig = await invoke("read_config");
  document.getElementById("sidetone").value = existingConfig["sidetone"];
  document.getElementById("lights").checked = existingConfig["lights"] === 1;

  const event = new Event('input');
  document.getElementById("sidetone").dispatchEvent(event);
  document.getElementById("lights").dispatchEvent(event);

  configFormEl.addEventListener("input", async () => {
      console.log("Form has changed!");
      const formData = new FormData(configFormEl);
      formData.append("sidetone", document.getElementById("sidetone").value);
      const lightsEl = document.getElementById("lights");
      const checkboxValue = lightsEl.checked ? 1 : 0;

      formData.append("lights", checkboxValue);
      const newConfig = {};
      formData.forEach((value, key) => newConfig[key] = parseInt(value));
      console.log(newConfig);
      await invoke("write_config", { headsetConfig: newConfig });
  });

  setInterval(async function(){ 
    await isHeadsetFound(configFormEl);   
  }, 1000);
});
