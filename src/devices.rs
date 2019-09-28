use crate::errors::SensorError;

pub trait Thermometer {
    fn temperature_celsius(&mut self) -> Result<f32, SensorError>;
}

pub trait Barometer {
    fn pressure_hpa(&mut self) -> Result<u32, SensorError>;
    fn humidity(&mut self) -> Result<f32, SensorError>;
}

pub trait AirQualitySensor {
    fn aqi(&mut self) -> Result<f32, SensorError>;
    fn gas_resistance(&mut self) -> Result<Option<u32>, SensorError>;
}
