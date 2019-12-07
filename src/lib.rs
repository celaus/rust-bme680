pub mod devices;
pub mod errors;
mod helpers;
mod sensors;
mod source;

use devices::{AirQualitySensor, Barometer, Thermometer};
use errors::SensorError;
use source::*;

use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;
use log::{debug, error, info, trace, warn};
use std::cell::RefCell;
use std::cmp;
use std::collections::BTreeMap;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

///
/// Over-sampling settings
///
pub enum Oversampling {
    None = 0,
    _1X = 1,
    _2X = 2,
    _4X = 3,
    _8X = 4,
    _16X = 5,
}

impl From<u8> for Oversampling {
    fn from(os: u8) -> Self {
        match os {
            1 => Oversampling::_1X,
            2 => Oversampling::_2X,
            4 => Oversampling::_4X,
            8 => Oversampling::_8X,
            16 => Oversampling::_16X,
            _ => Oversampling::None,
        }
    }
}

///
///  IIR filter settings
///
pub enum FilterSize {
    Size0 = 0,
    Size1 = 1,
    Size3 = 2,
    Size7 = 3,
    Size15 = 4,
    Size31 = 5,
    Size63 = 6,
    Size127 = 7,
}

impl From<u8> for FilterSize {
    fn from(filter: u8) -> Self {
        match filter {
            0 => FilterSize::Size0,
            1 => FilterSize::Size1,
            2 => FilterSize::Size3,
            3 => FilterSize::Size7,
            4 => FilterSize::Size15,
            5 => FilterSize::Size31,
            6 => FilterSize::Size63,
            _ => FilterSize::Size127,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Bme680Address {
    Primary = BME680_I2C_ADDR_PRIMARY as isize,
    Secondary = BME680_I2C_ADDR_SECONDARY as isize,
}

impl Default for Bme680Address {
    fn default() -> Self {
        Bme680Address::Primary
    }
}

thread_local!(static DEVICES: RefCell<BTreeMap<u8, LinuxI2CDevice>> = RefCell::new(BTreeMap::new()));

unsafe extern "C" fn write(dev_id: u8, reg_addr: u8, data: *mut u8, len: u16) -> i8 {
    DEVICES.with(|devices| {
        devices.borrow_mut().get_mut(&dev_id).map_or(1, |mut dev| {
            let d = std::slice::from_raw_parts(data, len as usize);
            dev.smbus_write_i2c_block_data(reg_addr, &d)
                .map(|_| 0)
                .map_err(|e| {
                    error!("error: {:?}", e);
                    e
                })
                .unwrap_or(1)
        })
    })
}

unsafe extern "C" fn read(dev_id: u8, reg_addr: u8, data: *mut u8, len: u16) -> i8 {
    DEVICES.with(|devices| {
        devices.borrow_mut().get_mut(&dev_id).map_or(1, |mut dev| {
            dev.smbus_read_i2c_block_data(reg_addr, len as u8)
                .map(|d| {
                    //let mut out_data = std::slice::from_raw_parts_mut(data,d.len());
                    ptr::copy_nonoverlapping(d.as_ptr(), data, cmp::min(len as usize, d.len()));
                    //out_data = &d.clone();
                    0
                })
                .unwrap_or(1)
        })
    })
}

unsafe extern "C" fn delay(ms: u32) {
    sleep(Duration::from_millis(ms as u64));
}

#[derive(Debug)]
pub struct Bme680Data {
    pub temperature: f32,
    pub pressure: u32,
    pub humidity: f32,
    pub gas_resistance: Option<u32>,
}

pub struct BME680 {
    native_device: bme680_dev,
    reset: bool,
    measure_period: u16,
    settings: u16,
}

impl BME680 {
    pub(crate) fn raw_init(dev: bme680_dev) -> BME680 {
        BME680 {
            native_device: dev,
            measure_period: 20, // some value, will be changed on first read
            reset: true,
            settings: BME680_OST_SEL
                | BME680_OSP_SEL
                | BME680_OSH_SEL
                | BME680_FILTER_SEL
                | BME680_GAS_SENSOR_SEL,
        }
    }

    pub fn initialize(device: &str, device_id: Bme680Address) -> Result<BME680, SensorError> {
        DEVICES.with(|devices_cell| {
            devices_cell.borrow_mut().insert(
                device_id as u8,
                LinuxI2CDevice::new(device, device_id as u16).unwrap(),
            );
        });

        let mut native_dev = bme680_dev {
            chip_id: BME680_CHIP_ID,
            dev_id: device_id as u8, // i2c address
            intf: bme680_intf_BME680_I2C_INTF,
            mem_page: 0,
            amb_temp: 25, // according to specs
            calib: bme680_calib_data::default(),
            tph_sett: bme680_tph_sett::default(),
            gas_sett: bme680_gas_sett::default(),
            power_mode: BME680_SLEEP_MODE, // sleep mode, 0x01 forced mode,
            new_fields: 0,
            info_msg: 0,
            read: Some(read),
            write: Some(write),
            delay_ms: Some(delay),
            com_rslt: 0,
        };

        let init_result;
        unsafe {
            init_result = bme680_init(&mut native_dev);
        }
        if init_result != BME680_OK {
            info!("failed to initialize '{}'", device);
            Err(SensorError::from(init_result))
        } else {
            info!("successfully initialized '{}'", device);
            Ok(BME680::raw_init(native_dev))
        }
    }

    fn activate_device(&mut self) -> Result<(), SensorError> {
        let mut rslt;
        let mut retries = 0;
        loop {
            self.native_device.power_mode = BME680_FORCED_MODE;

            unsafe {
                rslt = bme680_set_sensor_mode(&mut self.native_device);
                bme680_get_sensor_mode(&mut self.native_device);
            }
            if self.native_device.power_mode == BME680_FORCED_MODE {
                break;
            }
            retries += 1;
            unsafe {
                delay(10);
            }
        }
        debug!("Retrying setting the sensor to forced: took {} tries", retries);
        if rslt == BME680_OK {
            trace!("sensor set to FORCED");
            Ok(())
        } else {
            let e = SensorError::from(rslt);
            trace!("error setting sensor to forced: '{}'", e);
            Err(e)
        }
    }

    fn read_prep(&mut self) -> Result<(), SensorError> {
        let mut sleep_period = 20_u16;

        self.native_device.gas_sett.heatr_temp = 320;
        self.native_device.gas_sett.heatr_dur = 150;
        let rslt;
        unsafe {
            rslt = bme680_set_sensor_settings(self.settings, &mut self.native_device);
        }

        self.activate_device()?;

        unsafe {
            bme680_get_profile_dur(&mut sleep_period, &self.native_device);
        }

        self.measure_period = sleep_period;
        if rslt == BME680_OK {
            trace!("sensor prepared");
            self.reset = false;
            Ok(())
        } else {
            let e = SensorError::from(rslt);
            trace!("sensor preparation error: '{}'", e);
            Err(e)
        }
    }

    pub fn read_all(&mut self) -> Result<Bme680Data, SensorError> {
        let mut data = bme680_field_data::default();
        if self.reset {
            self.read_prep()?;
        } else {
            self.activate_device()?;
        }
        let rslt;
        unsafe {
            delay(self.measure_period as u32);
            rslt = bme680_get_sensor_data(&mut data, &mut self.native_device);
        }
        if rslt == BME680_OK {
            Ok(Bme680Data {
                pressure: data.pressure,
                temperature: data.temperature as f32 / 100.0,
                humidity: data.humidity as f32 / 1000.0,
                gas_resistance: if (data.status & BME680_GASM_VALID_MSK) == 0 {
                    Some(data.gas_resistance)
                } else {
                    None
                },
            })
        } else {
            let e = SensorError::from(rslt);
            trace!("error reading data: '{}'", e);
            Err(e)
        }
    }

    pub fn get_pressure_oversampling(&self) -> Oversampling {
        Oversampling::from(self.native_device.tph_sett.os_pres)
    }

    pub fn get_humidity_oversampling(&self) -> Oversampling {
        Oversampling::from(self.native_device.tph_sett.os_hum)
    }

    pub fn get_temperature_oversampling(&self) -> Oversampling {
        Oversampling::from(self.native_device.tph_sett.os_temp)
    }

    pub fn set_pressure_oversampling(&mut self, oversampling: Oversampling) {
        self.native_device.tph_sett.os_pres = oversampling as u8;
        self.reset = true;
    }

    pub fn set_humidity_oversampling(&mut self, oversampling: Oversampling) {
        self.native_device.tph_sett.os_hum = oversampling as u8;
        self.reset = true;
    }

    pub fn set_temperature_oversampling(&mut self, oversampling: Oversampling) {
        self.native_device.tph_sett.os_temp = oversampling as u8;
        self.reset = true;
    }

    pub fn get_filter_size(&self) -> FilterSize {
        FilterSize::from(self.native_device.tph_sett.filter)
    }

    pub fn set_filter_size(&mut self, filter: FilterSize) {
        self.native_device.tph_sett.filter = filter as u8;
        self.reset = true;
    }
    pub fn set_enable_gas_resistence(&mut self, enable: bool) {
        if enable {
            self.native_device.gas_sett.run_gas = BME680_ENABLE_GAS_MEAS;
        } else {
        }
    }

    pub fn get_gas_resistence(&self) -> bool {
        self.native_device.gas_sett.run_gas == BME680_ENABLE_GAS_MEAS
    }
}

impl Thermometer for BME680 {
    fn temperature_celsius(&mut self) -> Result<f32, SensorError> {
        self.read_all().map(|data| data.temperature)
    }
}

impl Barometer for BME680 {
    fn pressure_hpa(&mut self) -> Result<u32, SensorError> {
        self.read_all().map(|data| data.pressure)
    }

    fn humidity(&mut self) -> Result<f32, SensorError> {
        self.read_all().map(|data| data.humidity)
    }
}

impl AirQualitySensor for BME680 {
    fn gas_resistance(&mut self) -> Result<Option<u32>, SensorError> {
        self.read_all().map(|data| data.gas_resistance)
    }

    fn aqi(&mut self) -> Result<f32, SensorError> {
        self.read_all().map(|data| data.temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    thread_local!(static DATA: RefCell<BTreeMap<u8, LinuxI2CDevice>> = RefCell::new(BTreeMap::new()));

    unsafe extern "C" fn test_write(dev_id: u8, reg_addr: u8, data: *mut u8, len: u16) -> i8 {
        DATA.with(|data| data.borrow().get(&dev_id).map_or(1, |data| 0))
    }

    unsafe extern "C" fn test_read(dev_id: u8, reg_addr: u8, data: *mut u8, len: u16) -> i8 {
        DATA.with(|data| data.borrow().get(&dev_id).map_or(1, |data| 0))
    }

    unsafe extern "C" fn test_delay(ms: u32) {}

    fn fake_device(rw_result: u8) -> BME680 {
        BME680::raw_init(bme680_dev {
            chip_id: BME680_CHIP_ID,
            dev_id: rw_result, // i2c address
            intf: bme680_intf_BME680_I2C_INTF,
            mem_page: 0,
            amb_temp: 25, // according to specs
            calib: bme680_calib_data::default(),
            tph_sett: bme680_tph_sett::default(),
            gas_sett: bme680_gas_sett::default(),
            power_mode: BME680_SLEEP_MODE, // sleep mode, 0x01 forced mode,
            new_fields: 0,
            info_msg: 0,
            read: Some(test_read),
            write: Some(test_write),
            delay_ms: Some(test_delay),
            com_rslt: 0,
        })
    }

    #[test]
    fn read_temperature() {}
}
