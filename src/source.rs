#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/** BME680 General config */
pub const BME680_POLL_PERIOD_MS: u8 = 10;

/** BME680 I2C addresses */
pub const BME680_I2C_ADDR_PRIMARY: u8 = 0x76;
pub const BME680_I2C_ADDR_SECONDARY: u8 = 0x77;

/** BME680 unique chip identifier */
pub const BME680_CHIP_ID: u8 = 0x61;

/** BME680 coefficients related defines */
pub const BME680_COEFF_SIZE: u8 = 41;
pub const BME680_COEFF_ADDR1_LEN: u8 = 25;
pub const BME680_COEFF_ADDR2_LEN: u8 = 16;

/** BME680 field_x related defines */
pub const BME680_FIELD_LENGTH: u8 = 15;
pub const BME680_FIELD_ADDR_OFFSET: u8 = 17;

/** Soft reset command */
pub const BME680_SOFT_RESET_CMD: u8 = 0xb6;

/** Error code definitions */
pub const BME680_OK: i8 = 0;
/* Errors */
pub const BME680_E_NULL_PTR: i8 = -1;
pub const BME680_E_COM_FAIL: i8 = -2;
pub const BME680_E_DEV_NOT_FOUND: i8 = -3;
pub const BME680_E_INVALID_LENGTH: i8 = -4;

/* Warnings */
pub const BME680_W_DEFINE_PWR_MODE: i8 = 1;
pub const BME680_W_NO_NEW_DATA: i8 = 2;

/* Info's */
pub const BME680_I_MIN_CORRECTION: u8 = 1;
pub const BME680_I_MAX_CORRECTION: u8 = 2;

/** Register map */
/** Other coefficient's address */
pub const BME680_ADDR_RES_HEAT_VAL_ADDR: u8 = 0x00;
pub const BME680_ADDR_RES_HEAT_RANGE_ADDR: u8 = 0x02;
pub const BME680_ADDR_RANGE_SW_ERR_ADDR: u8 = 0x04;
pub const BME680_ADDR_SENS_CONF_START: u8 = 0x5A;
pub const BME680_ADDR_GAS_CONF_START: u8 = 0x64;

/** Field settings */
pub const BME680_FIELD0_ADDR: u8 = 0x1d;

/** Heater settings */
pub const BME680_RES_HEAT0_ADDR: u8 = 0x5a;
pub const BME680_GAS_WAIT0_ADDR: u8 = 0x64;

/** Sensor configuration registers */
pub const BME680_CONF_HEAT_CTRL_ADDR: u8 = 0x70;
pub const BME680_CONF_ODR_RUN_GAS_NBC_ADDR: u8 = 0x71;
pub const BME680_CONF_OS_H_ADDR: u8 = 0x72;
pub const BME680_MEM_PAGE_ADDR: u8 = 0xf3;
pub const BME680_CONF_T_P_MODE_ADDR: u8 = 0x74;
pub const BME680_CONF_ODR_FILT_ADDR: u8 = 0x75;

/** Coefficient's address */
pub const BME680_COEFF_ADDR1: u8 = 0x89;
pub const BME680_COEFF_ADDR2: u8 = 0xe1;

/** Chip identifier */
pub const BME680_CHIP_ID_ADDR: u8 = 0xd0;

/** Soft reset register */
pub const BME680_SOFT_RESET_ADDR: u8 = 0xe0;

/** Heater control settings */
pub const BME680_ENABLE_HEATER: u8 = 0x00;
pub const BME680_DISABLE_HEATER: u8 = 0x08;

/** Gas measurement settings */
pub const BME680_DISABLE_GAS_MEAS: u8 = 0x00;
pub const BME680_ENABLE_GAS_MEAS: u8 = 0x01;

/** Over-sampling settings */
pub const BME680_OS_NONE: u8 = 0;
pub const BME680_OS_1X: u8 = 1;
pub const BME680_OS_2X: u8 = 2;
pub const BME680_OS_4X: u8 = 3;
pub const BME680_OS_8X: u8 = 4;
pub const BME680_OS_16X: u8 = 5;

/** IIR filter settings */
pub const BME680_FILTER_SIZE_0: u8 = 0;
pub const BME680_FILTER_SIZE_1: u8 = 1;
pub const BME680_FILTER_SIZE_3: u8 = 2;
pub const BME680_FILTER_SIZE_7: u8 = 3;
pub const BME680_FILTER_SIZE_15: u8 = 4;
pub const BME680_FILTER_SIZE_31: u8 = 5;
pub const BME680_FILTER_SIZE_63: u8 = 6;
pub const BME680_FILTER_SIZE_127: u8 = 7;

/** Power mode settings */
pub const BME680_SLEEP_MODE: u8 = 0;
pub const BME680_FORCED_MODE: u8 = 1;

/** Delay related macro declaration */
pub const BME680_RESET_PERIOD: u32 = 10;

/** SPI memory page settings */
pub const BME680_MEM_PAGE0: u8 = 0x10;
pub const BME680_MEM_PAGE1: u8 = 0x00;

/** Ambient humidity shift value for compensation */
pub const BME680_HUM_REG_SHIFT_VAL: u8 = 4;

/** Run gas enable and disable settings */
pub const BME680_RUN_GAS_DISABLE: u8 = 0;
pub const BME680_RUN_GAS_ENABLE: u8 = 1;

/** Buffer length macro declaration */
pub const BME680_TMP_BUFFER_LENGTH: u8 = 40;
pub const BME680_REG_BUFFER_LENGTH: u8 = 6;
pub const BME680_FIELD_DATA_LENGTH: u8 = 3;
pub const BME680_GAS_REG_BUF_LENGTH: u8 = 20;

/** Settings selector */
pub const BME680_OST_SEL: u16 = 1;
pub const BME680_OSP_SEL: u16 = 2;
pub const BME680_OSH_SEL: u16 = 4;
pub const BME680_GAS_MEAS_SEL: u16 = 8;
pub const BME680_FILTER_SEL: u16 = 16;
pub const BME680_HCNTRL_SEL: u16 = 32;
pub const BME680_RUN_GAS_SEL: u16 = 64;
pub const BME680_NBCONV_SEL: u16 = 128;
pub const BME680_GAS_SENSOR_SEL: u16 =
    (BME680_GAS_MEAS_SEL | BME680_RUN_GAS_SEL | BME680_NBCONV_SEL);

/** Number of conversion settings*/
pub const BME680_NBCONV_MIN: u8 = 0;
pub const BME680_NBCONV_MAX: u8 = 10;

/** Mask definitions */
pub const BME680_GAS_MEAS_MSK: u8 = 0x30;
pub const BME680_NBCONV_MSK: u8 = 0x0F;
pub const BME680_FILTER_MSK: u8 = 0x1C;
pub const BME680_OST_MSK: u8 = 0xE0;
pub const BME680_OSP_MSK: u8 = 0x1C;
pub const BME680_OSH_MSK: u8 = 0x07;
pub const BME680_HCTRL_MSK: u8 = 0x08;
pub const BME680_RUN_GAS_MSK: u8 = 0x10;
pub const BME680_MODE_MSK: u8 = 0x03;
pub const BME680_RHRANGE_MSK: u8 = 0x30;
pub const BME680_RSERROR_MSK: u8 = 0xf0;
pub const BME680_NEW_DATA_MSK: u8 = 0x80;
pub const BME680_GAS_INDEX_MSK: u8 = 0x0f;
pub const BME680_GAS_RANGE_MSK: u8 = 0x0f;
pub const BME680_GASM_VALID_MSK: u8 = 0x20;
pub const BME680_HEAT_STAB_MSK: u8 = 0x10;
pub const BME680_MEM_PAGE_MSK: u8 = 0x10;
pub const BME680_SPI_RD_MSK: u8 = 0x80;
pub const BME680_SPI_WR_MSK: u8 = 0x7f;
pub const BME680_BIT_H1_DATA_MSK: u8 = 0x0F;

/** Bit position definitions for sensor settings */
pub const BME680_GAS_MEAS_POS: u8 = 4;
pub const BME680_FILTER_POS: u8 = 2;
pub const BME680_OST_POS: u8 = 5;
pub const BME680_OSP_POS: u8 = 2;
pub const BME680_RUN_GAS_POS: u8 = 4;
