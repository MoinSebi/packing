
use crate::convert::helper::{
    mean_vec_u16_u16, transform_u32_to_array_of_u8,
};
use bitvec::order::Msb0;
use bitvec::vec::BitVec;


/// VG pack representation + additional information.
///
/// Is working with VG version 1.3 (maybe also earlier)
pub struct PackCompact {
    pub node: Vec<u32>,                 // Node ids (also duplicated)
    pub coverage: Vec<u16>,             // Coverage of the nodes
    pub node_coverage: Vec<u16>,        // Coverage of nodes
    pub bin_coverage: BitVec<u8, Msb0>, // Binary coverage
    pub name: String,                   // Name of the pack/sample
}

impl PackCompact {
    /// PackCompact constructor.
    ///
    /// No arguments.
    pub fn new() -> Self {
        Self {
            node: Vec::new(),
            coverage: Vec::new(),
            node_coverage: Vec::new(),
            bin_coverage: BitVec::new(),
            name: String::new(),
        }
    }

    // Compression of data
    //------------------------------------------------------------------------------------------------------------------------------

    /// Compress only the nodes (index)
    ///
    /// - Convert node (u32) to [u8;4]
    /// - Then extend it to the buffer
    pub fn node2buffer(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len() {
            buffer.extend(transform_u32_to_array_of_u8(self.node[x]));
        }
        buffer
    }

    /// Calculate the average of the coverage for each node
    ///
    /// - Include 0
    /// - Add to struct
    /// - Always average method
    pub fn calc_node_cov(&mut self) {
        let mut node_id = self.node[0];
        let mut node_mean: Vec<u16> = Vec::new();
        let mut result: Vec<u16> = Vec::new();
        for x in 0..self.coverage.len() {
            if self.node[x] != node_id {
                result.push(mean_vec_u16_u16(&node_mean));

                node_id = self.node[x];
                node_mean = vec![self.coverage[x]];
            } else {
                node_mean.push(self.coverage[x])
            }
        }
        result.push(mean_vec_u16_u16(&node_mean));
        self.node_coverage = result
    }
}
