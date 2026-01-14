// use std::io::Write;
use std::thread;
use std::time::Duration;

use serialport::SerialPort;

mod arduino;
mod utils;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

fn main() {
    let arduino_port = match arduino::find() {
        Some(port) => port,
        None => {
            eprintln!("Arduino not found!");
            return;
        }
    };

    let mut port = serialport::new(&arduino_port, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .expect("Failed to open port");

    port.write_data_terminal_ready(false).ok();

    println!("Connected to Arduino on {}", arduino_port);

    thread::sleep(Duration::from_millis(500));

    run(port);
}

#[cfg(target_os = "windows")]
fn run(mut port: Box<dyn SerialPort>) {
    let mut last_volume = 255;
    let mut last_muted = false;

    loop {
        match windows::get_volume() {
            Ok((current_volume, is_muted)) => {
                if current_volume != last_volume || is_muted != last_muted {
                    let data = [current_volume, if is_muted { 1 } else { 0 }];

                    match port.write_all(&data) {
                        Ok(_) => {
                            port.flush().ok();
                            println!(
                                "{}",
                                if is_muted {
                                    format!("Muted (Volume: {}%)", current_volume)
                                } else {
                                    format!("Volume: {}%", current_volume)
                                }
                            );
                            last_volume = current_volume;
                            last_muted = is_muted;
                        }
                        Err(e) => {
                            eprintln!("Failed to send: {}", e);
                            thread::sleep(Duration::from_millis(500));
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed to get volume: {}", e),
        }

        thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(target_os = "linux")]
fn run(mut port: Box<dyn SerialPort>) {
    let mut last_volume = 255;
    let mut last_muted = false;

    loop {
        match linux::get_volume() {
            Ok((current_volume, is_muted)) => {
                if current_volume != last_volume || is_muted != last_muted {
                    let data = [current_volume, if is_muted { 1 } else { 0 }];

                    match port.write_all(&data) {
                        Ok(_) => {
                            port.flush().ok();
                            println!(
                                "{}",
                                if is_muted {
                                    format!("Muted (Volume: {}%)", current_volume)
                                } else {
                                    format!("Volume: {}%", current_volume)
                                }
                            );
                            last_volume = current_volume;
                            last_muted = is_muted;
                        }
                        Err(e) => {
                            eprintln!("Failed to send: {}", e);
                            thread::sleep(Duration::from_millis(500));
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed to get volume: {}", e),
        }

        thread::sleep(Duration::from_millis(100));
    }
}
