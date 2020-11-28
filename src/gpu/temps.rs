use std::{error::Error, fmt::Display, fs};

#[derive(Debug, Clone)]
pub struct Temperatures {
    pub edge: f32,
    pub junction: f32,
    pub memory: f32,
}

impl Temperatures {
    pub fn read_temps(hwmon_path: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Temperatures {
            edge: Temperatures::get_edge_temp(hwmon_path)?,
            junction: Temperatures::get_junction_temp(hwmon_path)?,
            memory: Temperatures::get_memory_temp(hwmon_path)?,
        })
    }

    fn get_edge_temp(hwmon_path: &str) -> Result<f32, Box<dyn Error>> {
        let power_consuption_file_path = format!("{}temp1_input", hwmon_path);
        let power_consuption_file_content = fs::read_to_string(power_consuption_file_path)?;
        let power_consuption = power_consuption_file_content.trim().parse::<f32>()? / 1000.0;
        Ok(power_consuption)
    }

    fn get_junction_temp(hwmon_path: &str) -> Result<f32, Box<dyn Error>> {
        let power_consuption_file_path = format!("{}temp2_input", hwmon_path);
        let power_consuption_file_content = fs::read_to_string(power_consuption_file_path)?;
        let power_consuption = power_consuption_file_content.trim().parse::<f32>()? / 1000.0;
        Ok(power_consuption)
    }

    fn get_memory_temp(hwmon_path: &str) -> Result<f32, Box<dyn Error>> {
        let power_consuption_file_path = format!("{}temp3_input", hwmon_path);
        let power_consuption_file_content = fs::read_to_string(power_consuption_file_path)?;
        let power_consuption = power_consuption_file_content.trim().parse::<f32>()? / 1000.0;
        Ok(power_consuption)
    }
}

impl Display for Temperatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Edge: {}ºC, Junction: {}ºC, Memory: {}ºC",
            self.edge, self.junction, self.memory
        )
    }
}
