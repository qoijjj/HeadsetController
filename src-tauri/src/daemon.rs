use std::process::Command;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use tokio::task;

#[derive(Deserialize, Serialize)]
struct Config {
    sidetone: Option<u32>,
    light: Option<u32>,
    preset: Option<u32>,
}

pub fn start() {
    dbg!("invoked");
    task::spawn(async {
        loop {
            // Check for the presence of the headset
            let output = Command::new("headsetcontrol")
            .output()
            .expect("Failed to execute command");
            
            let output_str = String::from_utf8_lossy(&output.stderr);
            if output_str.contains("No supported headset found") {
                dbg!("No supported headset found");
            } else {
                dbg!(output_str);
                 // Read or create config file
                let config_str = match std::fs::read_to_string("config.json") {
                    Ok(contents) => contents,
                    Err(_) => {
                        // If the file doesn't exist, create an empty JSON object
                        let empty_config = serde_json::to_string_pretty(&Config {
                            sidetone: None,
                            light: None,
                            preset: None,
                        })
                        .expect("Failed to serialize empty config");
                        std::fs::write("config.json", &empty_config).expect("Failed to write config file");
                        empty_config
                    }
                };

                match serde_json::from_str::<Config>(&config_str) {
                    Ok(config) => {
                        // Apply config based on values
                        if let Some(sidetone) = config.sidetone {
                            execute_headsetcontrol_command("-s", &sidetone.to_string());
                        }

                        if let Some(light) = config.light {
                            execute_headsetcontrol_command("-l", &light.to_string());
                        }

                        if let Some(preset) = config.preset {
                            execute_headsetcontrol_command("-p", &preset.to_string());
                        }
                    }
                    Err(err) => {
                        eprintln!("Error parsing config file: {}", err);
                    }
                }
            }

            // Sleep for 15 seconds before checking again
            sleep(Duration::from_secs(15)).await;
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
