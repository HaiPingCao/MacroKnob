# MK Driver (a.k.a macro knob driver)

## ⚠ I postponed the development of this driver due to a lack of time, and I am already satisfied with the current build.

A volume monitor that sends real-time audio volume data to Arduino via serial port.

## Features

- Monitors system volume in real-time
- Sends volume data to Arduino via serial connection
- Auto-detects Arduino port

## Requirements

- Rust (latest stable version)
- Arduino connected via USB and open serial port

## Usage

1. Connect your Arduino via USB
2. Upload the Arduino sketch to your board
3. Run the program:

```bash
cargo run
```

The program will:

- Auto-detect the Arduino port
- Connect at 9600 baud rate
- Send volume updates when changes are detected

## Data Format

The program sends 2 bytes per update:

- Byte 1: Volume level (0-100)
- Byte 2: Mute status (0 = unmuted, 1 = muted)

## Supported Platforms

- Windows
- Linux

## Troubleshooting

**Arduino not found**

- Ensure Arduino is connected via USB
- Check if Arduino drivers are installed

**Timeout errors**

- Volume changes are only sent when detected to prevent serial overload
- Check Arduino serial buffer if issues persist
- Only one program can access the serial port at a time
- Try close Arduino IDE Serial Monitor or any other program using the serial port
