use log::warn;

#[derive(Clone, Copy, PartialEq)]
pub enum OutputType {
    Node,
    Sequence,
    Pack,
}

impl OutputType {
    pub fn from_str(s: &str) -> OutputType {
        match s {
            "node" => OutputType::Node,
            "sequence" => OutputType::Sequence,
            "pack" => OutputType::Pack,
            _ => {
                warn!("Not one of the available output types");
                warn!("Using default value: node");
                OutputType::Node
            }
        }
    }

    pub fn from_u8(s: u8) -> OutputType {
        match s {
            0 => OutputType::Node,
            1 => OutputType::Sequence,
            2 => OutputType::Pack,
            _ => {
                warn!("Not one of the available output types");
                warn!("Using default value: node");
                OutputType::Node
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OutputType::Node => "Node".to_string(),
            OutputType::Sequence => "Sequence".to_string(),
            OutputType::Pack => "Pack".to_string(),
        }
    }
}

#[derive(PartialEq)]
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
