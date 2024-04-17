use log::warn;

#[derive(PartialEq, Copy, Clone)]
pub enum Method {
    Mean,
    Median,
    Percentile,
    Zscore,
    Nothing,
}

impl Method {
    pub fn from_str(s: &str) -> Method {
        match s {
            "mean" => Method::Mean,
            "median" => Method::Median,
            "nothing" => Method::Nothing,
            "percentile" => Method::Percentile,
            "zscore" => Method::Zscore,
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
            4 => Method::Zscore,
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
            Method::Zscore => "Zscore".to_string(),
        }
    }
}
