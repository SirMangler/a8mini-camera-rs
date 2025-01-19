# A8mini-gimbal-camera-control-rust

This project is a Rust port of A8mini-gimbal-camera-control (C version), which was originally created by thiagolages. 

During the porting process, certain architectural changes were made to better fit the Rust programming language
- Code for SIYI's A8 mini Gimbal Camera. Allows yaw/pitch control, as well as Auto Center, Zoom, etc.

## Setup

- Default IP is `192.168.144.25`
- Default port for control is `37260`

### List of currently supported commands:

- 0  - Auto Centering
- 1  - Rotate Up
- 2  - Rotate Down
- 3  - Rotate Right
- 4  - Rotate Left
- 5  - Stop rotation
- 6  - Zoom +1
- 7  - Zoom -1
- 8  - 4.5x
- 9  - Acquire the Max Zoom Value
- 10 - Manual Focus +1
- 11 - Manual Focus -1
- 12 - Take Pictures
- 13 - Record Video
- 14 - Rotate 100 100
- 15 - Gimbal Status Information
- 16 - Auto Focus
- 17 - Acquire Hardware ID
- 18 - Acquire Firmware Version
- 19 - Lock Mode
- 20 - Follow Mode
- 21 - FPV Mode
- 22 - Acquire Attitude Data
- 23 - Set Video Output as HDMI (Only available on A8 mini, restart to take effect)
- 24 - Set Video Output as CVBS (Only available on A8 mini, restart to take effect)
- 25 -  Turn Off both CVBS and HDMI Output (Only available on A8 mini, restart to take effect)
- 26 - Read Range from Laser Rangefinder(Low byte in the front, high byte in the back, available on ZT30)

**Note**: More commands might be supported by the camera but may not be included in the list of implemented commands.

**Disclamer**: SIYI does provide some sample code which was used to build this code.
