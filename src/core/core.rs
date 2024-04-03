use crate::convert::convert_helper::Method;
use crate::convert::helper::{mean_vec_u16_u16, transform_u32_to_array_of_u8};
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

/// VG pack representation + additional information.
///
/// Is working with VG version 1.3 (maybe also earlier)
pub struct PackCompact {
    pub node_index: Vec<u32>,                 // 4 bytesNode ids (also duplicated)
    pub coverage: Vec<u16>,             // 4 bytes - Coverage
    pub normalized_coverage: Vec<f32>,  // 2 bytes - coverage
    pub bin_coverage: BitVec<u8, Msb0>, // Binary coverage
    pub name: String,                   // Name of the pack/sample
    pub is_sequence: bool,
    pub is_binary: bool,
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
            is_binary: false,
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
        let mut node_id = self.node_index[0];
        let mut node_mean: Vec<u16> = Vec::new();
        let mut result: Vec<u16> = Vec::new();
        for x in 0..self.coverage.len() {
            if self.node_index[x] != node_id {
                result.push(mean_vec_u16_u16(&node_mean));

                node_id = self.node_index[x];
                node_mean = vec![self.coverage[x]];
            } else {
                node_mean.push(self.coverage[x])
            }
        }
        result.push(mean_vec_u16_u16(&node_mean));
        self.coverage = result
    }
}
