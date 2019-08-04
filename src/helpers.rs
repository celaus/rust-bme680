use crate::{bme680_calib_data, bme680_field_data, bme680_gas_sett, bme680_tph_sett};

impl Default for bme680_calib_data {
    fn default() -> Self {
        bme680_calib_data {
            par_h1: 0,
            par_h2: 0,
            par_h3: 0,
            par_h4: 0,
            par_h5: 0,
            par_h6: 0,
            par_h7: 0,
            par_gh1: 0,
            par_gh2: 0,
            par_gh3: 0,
            par_t1: 0,
            par_t2: 0,
            par_t3: 0,
            par_p1: 0,
            par_p2: 0,
            par_p3: 0,
            par_p4: 0,
            par_p5: 0,
            par_p6: 0,
            par_p7: 0,
            par_p8: 0,
            par_p9: 0,
            par_p10: 0,
            t_fine: 0,
            res_heat_range: 0,
            res_heat_val: 0,
            range_sw_err: 0,
        }
    }
}

impl Default for bme680_field_data {
    fn default() -> Self {
        bme680_field_data {
            status: 0,
            gas_index: 0,
            meas_index: 0,
            temperature: 0,
            pressure: 0,
            humidity: 0,
            gas_resistance: 0,
        }
    }
}

impl Default for bme680_tph_sett {
    fn default() -> Self {
        bme680_tph_sett {
            os_hum: 0,
            os_temp: 0,
            os_pres: 0,
            filter: 0,
        }
    }
}

impl Default for bme680_gas_sett {
    fn default() -> Self {
        bme680_gas_sett {
            nb_conv: 0,
            heatr_ctrl: 0,
            run_gas: 0,
            heatr_temp: 0,
            heatr_dur: 0,
        }
    }
}
