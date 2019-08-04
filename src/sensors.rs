use std::error::Error;
// Copy of the traits that used to come with the i2cdev library but are now gone ...

/// Trait for sensors that provide access to temperature readings
pub trait Thermometer {
    type Error: Error;

    /// Get a temperature from the sensor in degrees celsius
    ///
    /// Returns `Ok(temperature)` if available, otherwise returns
    /// `Err(Self::Error)`
    fn temperature_celsius(&mut self) -> Result<f32, Self::Error>;
}

/// Trait for sensors that provide access to pressure readings
pub trait Barometer {
    type Error: Error;

    /// Get a pressure reading from the sensor in kPa
    ///
    /// Returns `Ok(temperature)` if avialable, otherwise returns
    /// `Err(Self::Error)`
    fn pressure_kpa(&mut self) -> Result<f32, Self::Error>;
}

pub trait Humidity {
    type Error: Error;

    /// Get a pressure reading from the sensor in kPa
    ///
    /// Returns `Ok(temperature)` if avialable, otherwise returns
    /// `Err(Self::Error)`
    fn relative_humidity(&mut self) -> Result<f32, Self::Error>;
}

pub trait AirQuality {
    type Error: Error;

    /// Get a pressure reading from the sensor in kPa
    ///
    /// Returns `Ok(temperature)` if avialable, otherwise returns
    /// `Err(Self::Error)`
    fn resistance_ohms(&mut self) -> Result<f32, Self::Error>;

    fn aqi(&mut self) -> Result<f32, Self::Error>;
}
