use crate::convert::convert_helper::Method;
use crate::convert::helper::{
    calculate_std_deviation, mean, mean_vec_u16_f64, median, median_vec_u16_16,
    remove_zero, remove_zero_f32, transform_u32_to_array_of_u8,
};
use crate::core::core::DataType::{TypeBit, TypeF32, TypeU16};
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use log::{info, warn};

#[derive(PartialEq)]
pub enum DataType {
    TypeF32,
    TypeBit,
    TypeU16,
}

impl DataType {
    pub fn fromU8(input: u8) -> Self {
        if input == 0 {
            TypeU16
        } else if input == 1 {
            TypeBit
        } else {
            TypeF32
        }
    }

    pub fn toU8(&self) -> u8 {
        if self == &TypeU16 {
            0
        } else if self == &TypeBit {
            1
        } else {
            2
        }
    }

    pub fn to_string1(&self) -> String {
        if self == &TypeU16 {
            "u16".to_string()
        } else if self == &TypeBit {
            "bit".to_string()
        } else {
            "f32".to_string()
        }
    }
}

/// VG pack representation + additional information.
///
/// Is working with VG version 1.3 (maybe also earlier)
pub struct PackCompact {
    pub node_index: Vec<u32>,           // 4 bytesNode ids (also duplicated)
    pub coverage: Vec<u16>,             // 4 bytes - Coverage
    pub normalized_coverage: Vec<f32>,  // 2 bytes - coverage
    pub bin_coverage: BitVec<u8, Msb0>, // Binary coverage
    pub name: String,                   // Name of the pack/sample
    pub is_sequence: bool,
    pub is_binary: DataType,
    pub method: Method,
    pub fraction: f32,
    pub std: f32,
    pub threshold: f32,
    pub length: u32,
}

impl Default for PackCompact {
    fn default() -> Self {
        Self::new()
    }
}

impl PackCompact {
    /// PackCompact constructor.
    ///
    /// No arguments.
    pub fn new() -> Self {
        Self {
            node_index: Vec::new(),
            coverage: Vec::new(),
            normalized_coverage: Vec::new(),
            bin_coverage: BitVec::new(),
            name: String::new(),
            is_sequence: false,
            is_binary: DataType::TypeU16,
            method: Method::Nothing,
            fraction: 0.0,
            std: 0.0,
            threshold: 0.0,
            length: 0,
        }
    }

    // Compression of data
    //------------------------------------------------------------------------------------------------------------------------------

    /// Compress only the nodes (index)
    ///
    /// - Convert node (u32) to [u8;4]
    /// - Then extend it to the buffer
    pub fn node_index2buffer(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len() {
            buffer.extend(transform_u32_to_array_of_u8(self.node_index[x]));
        }
        buffer
    }

    /// Calculate the average of the coverage for each node
    ///
    /// - Include 0
    /// - Add to struct
    /// - Always average method
    pub fn calc_node_cov(&mut self) {
        if self.normalized_coverage.is_empty(){
            let mut node_id = self.node_index[0];
            let mut node_mean: Vec<u16> = Vec::new();
            let mut result: Vec<f32> = Vec::new();
            for x in 0..self.coverage.len() {
                if self.node_index[x] != node_id {
                    result.push(mean(&node_mean) as f32);

                    node_id = self.node_index[x];
                    node_mean = vec![self.coverage[x]];
                } else {
                    node_mean.push(self.coverage[x])
                }
            }
            result.push(mean(&node_mean) as f32);
            self.normalized_coverage = result
        } else {
            let mut node_id = self.node_index[0];
            let mut node_mean: Vec<f32> = Vec::new();
            let mut result: Vec<f32> = Vec::new();
            for x in 0..self.normalized_coverage.len() {
                if self.node_index[x] != node_id {
                    result.push(mean(&node_mean) as f32);

                    node_id = self.node_index[x];
                    node_mean = vec![self.normalized_coverage[x]];
                } else {
                    node_mean.push(self.normalized_coverage[x])
                }
            }
            result.push(mean(&node_mean) as f32);
            self.normalized_coverage = result
        }
    }

    pub fn get_threshold(&self, include_all: bool, relative: f32, std: f32, tt: Method) -> f32 {
        if self.normalized_coverage.is_empty() {
            // "work_on" is the current data we do the normalizcation on
            let mut work_on: Vec<u16> = self.coverage.clone();
            // relative is 0
            if relative == 0.0 {
                warn!("Relative threshold is 0");
                return 0.0;
            }
            if !include_all {
                remove_zero(&mut work_on)
            }

            let mut a_std = 0.0;
            if std != 0.0 {
                a_std = calculate_std_deviation(&work_on) as f32
            }

            let mut thresh: f32 = 0.0;
            if tt == Method::Percentile {
                work_on.sort();
                thresh = work_on[((work_on.len() as f32 - 1.0) * relative).round() as usize] as f32;
                info!("{}% Percentile is {}", relative, thresh);
                info!("Working threshold is {}", thresh);
                return thresh;
            } else if tt == Method::Mean {
                thresh = mean_vec_u16_f64(&work_on) as f32;
                info!("Mean is {}", thresh);
            } else if tt == Method::Median {
                thresh = median_vec_u16_16(&work_on) as f32;
                info!("Median is {}", thresh);
            }
            thresh -= std;
            info!("Working threshold is {}", thresh);
            thresh *= relative;
            info!("Working threshold is {}", thresh);

            thresh
        } else {
            let mut work_on: Vec<f32> = self.normalized_coverage.clone();
            // relative is 0
            if relative == 0.0 {
                warn!("Relative threshold is 0");
                return 0.0;
            }

            if !include_all {
                remove_zero_f32(&mut work_on)
            }

            let mut thresh: f32 = 0.0;
            if tt == Method::Percentile {
                work_on.sort_by(|a, b| a.partial_cmp(b).unwrap());
                thresh = work_on[((work_on.len() as f32 - 1.0) * relative).round() as usize];
                info!("{}% Percentile is {}", relative, thresh);
                info!("Working threshold is {}", thresh);
                return thresh;
            } else if tt == Method::Mean {
                thresh = mean(&work_on) as f32;
                info!("Mean is {}", thresh);
                info!("Working threshold is {}", thresh);
            } else if tt == Method::Median {
                thresh = median(&mut work_on) as f32;
                info!("Median is {}", thresh);
                info!("Working threshold is {}", thresh);
            }
            thresh -= std;
            info!("Working threshold is {}", thresh);
            thresh *= relative;
            info!("Working threshold is {}", thresh);

            thresh
        }
    }
}
