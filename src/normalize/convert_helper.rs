use log::warn;

#[derive(PartialEq, Copy, Clone)]
pub enum Method {
    Mean,
    Median,
    Percentile,
    Nothing,
    Absolute,
    Compress,
}

impl Method {
    pub fn from_str(s: &str) -> Method {
        match s {
            "mean" => Method::Mean,
            "median" => Method::Median,
            "compress" => Method::Compress,
            "percentile" => Method::Percentile,
            "absolute" => Method::Absolute,
            "nothing" => Method::Nothing,
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
            4 => Method::Absolute,
            5 => Method::Compress,
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
            Method::Absolute => "Absolute".to_string(),
            Method::Compress => "Compress".to_string(),
        }
    }
}
