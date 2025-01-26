
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum A8MiniComplexCommand {
    SetYawPitchSpeed(u16, u16),
    SetYawSpeed(u16),
    SetPitchSpeed(u16),
    SetYawPitchAngle(u16, u16),
    SetYawAngle(u16),
    SetPitchAngle(u16),
}


pub enum A8MiniCommandResponse {
    Failure = 0,
    Success = 1,
}

pub enum A8MiniHttpQuery {
    
}




