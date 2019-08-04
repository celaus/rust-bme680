mod errors;
mod helpers;
mod sensors;
mod source;

use errors::SensorError;
use source::*;

use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;
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

///
///  IIR filter settings
///
pub enum FilterSize {
    Size_0 = 0,
    Size_1 = 1,
    Size_3 = 2,
    Size_7 = 3,
    Size_15 = 4,
    Size_31 = 5,
    Size_63 = 6,
    Size_127 = 7,
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
                    println!("error: {:?}", e);
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

pub struct Bme680Data {
    pub temperature: f32,
    pub pressure: u32,
    pub humidity: f32,
    pub gas_resistance: u32,
}

pub struct BME680 {
    native_device: bme680_dev,
}

impl BME680 {
    pub fn initialize(device: &str, device_id: Bme680Address) -> Result<BME680, SensorError> {
        let _ = DEVICES.with(|devices_cell| {
            devices_cell
                .borrow_mut()
                .insert(
                    device_id as u8,
                    LinuxI2CDevice::new(device, device_id as u16).unwrap(),
                )
                .unwrap()
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

        let mut init_result = BME680_OK;
        unsafe {
            init_result = bme680_init(&mut native_dev);
        }
        if init_result != BME680_OK {
            Err(SensorError::from(init_result))
        } else {
            Ok(BME680 {
                native_device: native_dev,
            })
        }
    }

    fn read_prep(&mut self, sleep: u32) -> Result<(), SensorError> {
        let set_required_settings = BME680_OST_SEL
            | BME680_OSP_SEL
            | BME680_OSH_SEL
            | BME680_FILTER_SEL
            | BME680_GAS_SENSOR_SEL;

        self.native_device.gas_sett.heatr_temp = 320;
        self.native_device.gas_sett.heatr_dur = 150;
        let mut rslt = BME680_OK;
        unsafe {
            rslt = bme680_set_sensor_settings(set_required_settings, &mut self.native_device);
            bme680_get_profile_dur(&mut (sleep as u16), &self.native_device);
        }
        if BME680_OK == rslt {
            Ok(())
        } else {
            Err(SensorError::from(rslt))
        }
    }

    pub fn read_all(&mut self) -> Result<Bme680Data, SensorError> {
        let mut data = bme680_field_data::default();
        let meas_period = 20;
        let _ = self.read_prep(meas_period)?;
        let mut rslt = BME680_OK;
        unsafe {
            delay(meas_period);
            rslt = bme680_get_sensor_data(&mut data, &mut self.native_device);
        }
        if rslt == BME680_OK {
            Ok(Bme680Data {
                pressure: data.pressure,
                temperature: data.temperature as f32 / 100.0,
                humidity: data.humidity as f32 / 1000.0,
                gas_resistance: data.gas_resistance,
            })
        } else {
            Err(SensorError::from(rslt))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
