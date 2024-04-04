use crate::core::core::PackCompact;
use log::warn;


#[derive(PartialEq, Copy, Clone)]
pub enum Method {
    Mean,
    Median,
    Percentile,
    Nothing,
}

impl Method {
    pub fn from_str(s: &str) -> Method {
        match s {
            "mean" => Method::Mean,
            "median" => Method::Median,
            "nothing" => Method::Nothing,
            "percentile" => Method::Percentile,
            _ => {
                warn!("Not one of the available methods");
                warn!("Using default value: nothing");
                Method::Nothing
            }
        }
    }

    pub fn from_u8(s: u8) -> Method {
        match s {
            0 => Method::Nothing,
            1 => Method::Mean,
            2 => Method::Median,
            3 => Method::Percentile,
            _ => {
                warn!("Not one of the available methods");
                warn!("Using default value: nothing");
                Method::Nothing
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Method::Mean => "Mean".to_string(),
            Method::Median => "Median".to_string(),
            Method::Nothing => "Nothing".to_string(),
            Method::Percentile => "Percentile".to_string(),
        }
    }
}

impl PackCompact {
    fn calculate_std(&self) -> Option<f64> {
        // Ensure there is at least one element in the vector
        if self.coverage.is_empty() {
            return None;
        }

        // Calculate the mean
        let sum: u32 = self.coverage.iter().map(|&x| x as u32).sum();
        let mean: f64 = sum as f64 / self.coverage.len() as f64;

        // Calculate the sum of squared differences from the mean
        let sum_squared_diff: f64 = self
            .coverage
            .iter()
            .map(|&x| ((x as f64) - mean).powi(2))
            .sum();

        // Calculate the variance and return the standard deviation
        Some(sum_squared_diff.sqrt() / self.coverage.len() as f64)
    }
}
