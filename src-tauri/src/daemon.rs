use tokio::time::{sleep, Duration};
use tokio::task;
use std::process::Command;
use crate::utils;

pub fn start() {
    task::spawn(async {
        let mut previous_sidetone: Option<u32> = None;
        let mut previous_lights: Option<u32> = None;

        loop {
            if utils::is_headset_found() {
                if let Some(config) = utils::read_config() {
                    if !config.sidetone.is_none() {
                        if previous_sidetone.is_none() || previous_sidetone.unwrap() != config.sidetone.unwrap() {
                            execute_headsetcontrol_command("-s".to_string(), config.sidetone.unwrap().to_string());
                            previous_sidetone = config.sidetone;
                        }
                    }

                    if !config.lights.is_none() {
                        if previous_lights.is_none() || previous_lights.unwrap() != config.lights.unwrap() {
                            execute_headsetcontrol_command("-l".to_string(), config.lights.unwrap().to_string());
                            previous_lights = config.lights;
                        }
                    }
                }
            }

            // Sleep for 15 seconds before checking again
            sleep(Duration::from_secs(3)).await;
        }
    });
}

fn execute_headsetcontrol_command(arg: String, value: String) {
    let command = Command::new("headsetcontrol")
        .arg(arg.clone())
        .arg(value.clone())
        .output()
        .expect("Failed to execute command");
}
