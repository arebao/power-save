use std::{env::args, process::Command};

fn main() {
    let mut iw = Command::new("iw");
    match args().nth(1) {
        Some(switch) => match switch.to_lowercase().as_str() {
            "on" => {
                iw.arg("dev wlan0 set power_save on")
                    .output()
                    .expect("There was an error running the command.")
                    .stdout;
            }
            "off" => {
                iw.arg("dev wlan0 set power_save off")
                    .output()
                    .expect("There was an error running the command.")
                    .stdout;
            }
            _ => {}
        },
        None => {
            println!("Use power-save on|off to switch wifi power-saving mode");
        }
    }
    let result = iw
        .arg("dev wlan0 get power_save")
        .output()
        .expect("There was an error running the command.")
        .stdout;
    println!(
        "Current power-save value is {:?}.",
        String::from_utf8(result)
    );
}
