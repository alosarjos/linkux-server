use std::{error::Error, fmt::Display, fs};

use super::temps::Temperatures;
#[derive(Debug)]
pub struct Status {
    voltage: f32,
    power_consumption: f32,
    fan_speed: i32,
    temperatures: Option<Temperatures>,
}

impl Status {
    pub fn read_status(hwmon_path: &str) -> Result<Status, Box<dyn Error>> {
        Ok(Status {
            voltage: Status::get_current_voltage(hwmon_path)?,
            power_consumption: Status::get_power_consuption(hwmon_path)?,
            fan_speed: Status::get_fan_speed(hwmon_path)?,
            temperatures: match Temperatures::read_temps(hwmon_path) {
                Ok(temps) => Some(temps),
                Err(_) => None,
            },
        })
    }

    fn get_current_voltage(hwmon_path: &str) -> Result<f32, Box<dyn Error>> {
        let voltage_file_path = format!("{}in0_input", hwmon_path);
        let voltage_file_content = fs::read_to_string(voltage_file_path)?;
        let current_voltage = voltage_file_content.trim().parse()?;
        Ok(current_voltage)
    }

    fn get_power_consuption(hwmon_path: &str) -> Result<f32, Box<dyn Error>> {
        let power_consuption_file_path = format!("{}power1_average", hwmon_path);
        let power_consuption_file_content = fs::read_to_string(power_consuption_file_path)?;
        let power_consuption = power_consuption_file_content.trim().parse::<f32>()? / 1000000.0;
        Ok(power_consuption)
    }

    fn get_fan_speed(hwmon_path: &str) -> Result<i32, Box<dyn Error>> {
        let fan_speed_file_path = format!("{}fan1_input", hwmon_path);
        let fan_speed_file_content = fs::read_to_string(fan_speed_file_path)?;
        let fan_speed = fan_speed_file_content.trim().parse()?;
        Ok(fan_speed)
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Voltage: {} mV, Power consumption: {} W, Fan Speed: {} RPM\nTemperatures: {}",
            self.voltage,
            self.power_consumption,
            self.fan_speed,
            self.temperatures.clone().unwrap()
        )
    }
}
