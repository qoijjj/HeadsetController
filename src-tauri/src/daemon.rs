use tokio::time::{sleep, Duration};
use tokio::task;
use std::process::Command;
use crate::utils;

pub fn start() {
    task::spawn(async {
        loop {
            if utils::is_headset_found() {
                if let Some(config) = utils::read_config() {
                    if let Some(sidetone) = config.sidetone {
                        execute_headsetcontrol_command("-s", &sidetone.to_string());
                    }

                    if let Some(light) = config.lights {
                        execute_headsetcontrol_command("-l", &light.to_string());
                    }

                    if let Some(preset) = config.preset {
                        execute_headsetcontrol_command("-p", &preset.to_string());
                    }
                }
            }

            // Sleep for 15 seconds before checking again
            sleep(Duration::from_secs(3)).await;
        }
    });
}

fn execute_headsetcontrol_command(arg: &str, value: &str) {
    let command = Command::new("headsetcontrol")
        .arg(arg)
        .arg(value)
        .output()
        .expect("Failed to execute command");

    if !command.status.success() {
        eprintln!(
            "Failed to execute 'headsetcontrol {} {}'",
            arg, value
        );
    }
}
