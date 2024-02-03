use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct HeadsetConfig {
    pub sidetone: Option<u32>,
    pub lights: Option<u32>,
    pub preset: Option<u32>,
}

fn get_config_file_name() -> String {
     return "headset_config.json".to_string();
}

pub fn is_headset_found() -> bool {
     // Check for the presence of the headset
     let output = Command::new("headsetcontrol")
     .output()
     .expect("Failed to execute command");
     
     let output_str = String::from_utf8_lossy(&output.stderr);
     return !output_str.contains("No supported headset found")
}

pub fn write_config(headset_config: HeadsetConfig) {
     let headset_config_str_json = serde_json::to_string::<HeadsetConfig>(&headset_config)
          .unwrap_or_else(|err| panic!("Failed to serialize config: {}", err));
  
     dbg!(headset_config_str_json.clone());
     fs::write(get_config_file_name(), headset_config_str_json).expect("Failed to write config file");
}

pub fn read_config() -> Option<HeadsetConfig> {
     let headset_config_str_json = fs::read_to_string(get_config_file_name()).ok()?;
     return serde_json::from_str::<HeadsetConfig>(&headset_config_str_json).ok();
}