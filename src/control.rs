use crate::{checksum, constants};


pub trait Command {
    fn to_bytes(self) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum A8MiniSimpleCommand {
    AutoCenter = 0, // handled ACK (sta)
    RotateUp = 1, // handled ACK (sta)
    RotateDown = 2, // handled ACK (sta)
    RotateRight = 3, // handled ACK (sta)
    RotateLeft = 4, // handled ACK (sta)
    StopRotation = 5, // handled ACK (sta)
    ZoomIn = 6, // handled ACK (sta)
    ZoomOut = 7, // handled ACK (sta)
    ZoomMax = 8,
    MaxZoomInformation = 9,
    FocusIn = 10,
    FocusOut = 11,
    TakePicture = 12, // no ACK
    RecordVideo = 13, // no ACK
    Rotate100100 = 14,
    CameraInformation = 15,
    AutoFocus = 16, // handled ACK (sta)
    HardwareIDInformation = 17,
    FirmwareVersionInformation = 18,
    SetLockMode = 19,
    SetFollowMode = 20,
    SetFPVMode = 21,
    AttitudeInformation = 22,
    SetVideoOutputHDMI = 23,
    SetVideoOutputCVBS = 24,
    SetVideoOutputOff = 25,
    LaserRangefinderInformation = 26,
    RebootCamera = 27,
    RebootGimbal = 28,
}

impl Command for A8MiniSimpleCommand {
    fn to_bytes(self) -> Vec<u8> {
        constants::HARDCODED_COMMANDS[self as usize].to_vec()
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum A8MiniComplexCommand {
    SetYawPitchSpeed(i8, i8),
    SetYawPitchAngle(i16, i16),
}

impl Command for A8MiniComplexCommand {
    fn to_bytes(self) -> Vec<u8> {
        match self {
            A8MiniComplexCommand::SetYawPitchSpeed(v_yaw, v_pitch) => {
                let mut byte_arr: Vec<u8> = vec![0x55,0x66,0x01,0x02,0x00,0x00,0x00,0x07];

                byte_arr.push(v_yaw.clamp(-100, 100) as u8);
                byte_arr.push(v_pitch.clamp(-100, 100) as u8);

                byte_arr.extend_from_slice(&checksum::crc16_calc(&byte_arr, 0));

                byte_arr
            },
            A8MiniComplexCommand::SetYawPitchAngle(theta_yaw, theta_pitch) => {
                let mut byte_arr: Vec<u8> = vec![0x55,0x66,0x01,0x02,0x00,0x00,0x00,0x0E];

                byte_arr.push(theta_yaw.clamp(-135, 135) as u8);
                byte_arr.push(theta_pitch.clamp(-90, 25) as u8);

                byte_arr.extend_from_slice(&checksum::crc16_calc(&byte_arr, 0));
                
                byte_arr
            },
        }
    }
}




