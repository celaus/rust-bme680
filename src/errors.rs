use crate::source::{
    BME680_E_COM_FAIL, BME680_E_DEV_NOT_FOUND, BME680_E_INVALID_LENGTH, BME680_E_NULL_PTR,
};

#[derive(Copy, Clone, Debug)]
pub enum SensorError {
    CommunicationError = BME680_E_COM_FAIL as isize,
    DeviceNotFound = BME680_E_DEV_NOT_FOUND as isize,
    InvalidLength = BME680_E_INVALID_LENGTH as isize,
    NullPointer = BME680_E_NULL_PTR as isize,
    Unknown,
}

impl std::error::Error for SensorError {}

impl std::fmt::Display for SensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            SensorError::CommunicationError => {
                format!("Communication Error, code '{}'", *self as u8)
            }
            SensorError::DeviceNotFound => format!("Device not found, code '{}'", *self as u8),
            SensorError::InvalidLength => format!("Invalid length, code '{}'", *self as u8),
            SensorError::NullPointer => {
                format!("Internal Null Pointer encountered, code '{}'", *self as u8)
            }
            SensorError::Unknown => format!("An unknown error occurred, code '{}'", *self as u8),
        };
        write!(f, "{}", &msg)
    }
}

impl From<i8> for SensorError {
    fn from(error: i8) -> Self {
        match error {
            error if error == SensorError::CommunicationError as i8 => {
                SensorError::CommunicationError
            }
            error if error == SensorError::DeviceNotFound as i8 => SensorError::DeviceNotFound,
            error if error == SensorError::InvalidLength as i8 => SensorError::InvalidLength,
            error if error == SensorError::NullPointer as i8 => SensorError::NullPointer,
            _ => SensorError::Unknown,
        }
    }
}
