use bitvec::vec::BitVec;
use byteorder::BigEndian;
use log::info;
use crate::helper::{binary2u8, vec_u16_u82, transform_u32_to_array_of_u8, u8_u322, mean_vecU16_u16, mean_vec_u32, median, mean_vec_f32, vec_f32_u82};
use crate::reader::get_file_as_byte_vec;

/// VG pack representation + additional information.
///
/// Is working with VG version 1.3 (maybe also earlier)
pub struct PackCompact {
    pub node: Vec<u32>,
    pub coverage: Vec<u16>,
    pub coverage_normalized: Vec<f32>
}

impl PackCompact {
    /// PackCompact constructor.
    ///
    /// No arguments.
    pub fn new() -> Self {
        Self {
            node: Vec::new(),
            coverage: Vec::new(),
            coverage_normalized: Vec::new()
        }
    }






    //----------------------------------------------------------------------------------------------------------
    // Reduce the data for packing output




    pub fn node2byte_thresh(&self, thresh: &u16) -> Vec<bool>{
        let mut node_id = 1;
        let mut node_mean: Vec<u16> = Vec::new();
        let mut result: Vec<bool> = Vec::new();
        for x in 0..self.coverage.len(){
            if self.node[x] != node_id {
                let mm = mean_vecU16_u16(&node_mean);
                if &mm >= thresh{
                    result.push(true);
                } else {
                    result.push(false);
                }
                node_id = self.node[x] ;
                node_mean = vec![self.coverage[x] as u16];
            } else {
                node_mean.push(self.coverage[x] as u16)
            }
        }
        result

    }

    pub fn node2byte(&self) -> Vec<u16>{
        let mut node_id = 1;
        let mut node_mean: Vec<u16> = Vec::new();
        let mut result: Vec<u16> = Vec::new();
        println!("{}", self.coverage.len());
        for x in 0..self.coverage.len() {
            if self.node[x] != node_id {
                result.push(mean_vecU16_u16(&node_mean));

                node_id = self.node[x];
                node_mean = vec![self.coverage[x] as u16];
            } else {
                node_mean.push(self.coverage[x] as u16)
            }
        }
        println!("{}", node_mean.len());
        result.push(mean_vecU16_u16(&node_mean));
        result

    }

    pub fn node2byte_thresh_normalized(&self, thresh: &f32) -> Vec<bool>{
        let mut node_id = 1;
        let mut node_mean: Vec<f32> = Vec::new();
        let mut result: Vec<bool> = Vec::new();
        for x in 0..self.coverage_normalized.len(){
            if self.node[x] != node_id {
                let mm = mean_vec_f32(&node_mean);
                if &mm >= thresh{
                    result.push(true);
                } else {
                    result.push(false);
                }
                node_id = self.node[x] ;
                node_mean = vec![self.coverage_normalized[x] as f32];
            } else {
                node_mean.push(self.coverage_normalized[x] as f32)
            }
        }
        result

    }


    pub fn node2byte_normalized(&self) -> Vec<f32>{
        let mut node_id = 1;
        let mut node_mean: Vec<f32> = Vec::new();
        let mut result: Vec<f32> = Vec::new();
        for x in 0..self.coverage.len(){
            if self.node[x]  != node_id {
                result.push(mean_vec_f32(&node_mean));

                node_id = self.node[x] ;
                node_mean = vec![self.coverage_normalized[x] ];
            } else {
                node_mean.push(self.coverage_normalized[x])
            }
        }
        result

    }




    /// Coverage vector to byte vector
    /// Storing only bits
    pub fn coverage2byte_thresh_bit(&self, thresh: &u16) -> Vec<u8> {
        let mut j: Vec<bool> = Vec::new();
        for x in self.coverage.iter() {
            if *x as u16 >= *thresh {
                j.push(true)
            } else {
                j.push(false);
            }
        }
        let h = binary2u8(&j);
        h
    }

    // NORMALIZED

    pub fn coverage2byte_normalized(&self) -> Vec<u8> {
        let h = vec_f32_u82(&self.coverage_normalized);
        h
    }


    /// Coverage vector to byte vector
    /// Storing only u16 for max coverage
    pub fn coverage2byte(&self) -> Vec<u8> {
        let h = vec_u16_u82(&self.coverage);
        h
    }


    pub fn coverage2byte_thresh_normalized(&self, thresh: &f32) -> Vec<u8>{
        let mut j: Vec<bool> = Vec::new();
        for x in self.coverage_normalized.iter() {
            if x >= thresh {
                j.push(true)
            } else {
                j.push(false);
            }
        }
        let h = binary2u8(&j);
        h

    }




    // Compression of data
    //------------------------------------------------------------------------------------------------------------------------------

    /// Compress only the nodes (index)
    pub fn compress_only_node(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len() {
            buf.extend(transform_u32_to_array_of_u8(self.node[x]));
        }
        //println!("1 {}", buf.len());

        buf
    }




    // #[allow(dead_code)]
    // // This might be overkill - keep it for later
    // pub fn compress_smart(&self) -> Vec<u8>{
    //     let mut buf: Vec<u8> = Vec::new();
    //     let mut node: &u32 = &0;
    //     let mut repeats: u32 = 0;
    //     let mut cov: Vec<u32> = Vec::new();
    //
    //     for x in 0..self.coverage.len(){
    //        if (self.node[x] != *node) & (repeats != 0){
    //            buf.extend(transform_u32_to_array_of_u8(node.clone()));
    //            buf.extend(transform_u32_to_array_of_u8(repeats));
    //            buf.extend(vec_u16_u82(&cov));
    //            node = &self.node[x];
    //            repeats = 1;
    //            cov = vec![self.coverage[x]];
    //
    //        }
    //         else {
    //             repeats += 1;
    //             cov.push(self.coverage[x].clone());
    //             node = &self.node[x];
    //         }
    //     }
    //     buf.extend(transform_u32_to_array_of_u8(node.clone()));
    //     buf.extend(transform_u32_to_array_of_u8(repeats));
    //     buf.extend(vec_u16_u82(&cov));
    //     //eprintln!("2 {}", buf.len());
    //     buf
    // }

    pub fn get_real_threshold(&self, node: bool, all: bool, relative: u16, tt: &str) -> u16{
        let mut work_on: Vec<u16>;
        if node{
            work_on = self.get_node_cov_mean();

        } else {
            work_on = self.coverage.clone();
        }

         if all{
             remove_zero(& mut work_on)
         } else {
             remove_zero(& mut work_on)
         }
        let mut thresh = 0;
        if tt != "nothing"{
            work_on.sort();
            thresh = work_on[(((work_on.len() as f64)/(100 as f64)) * ((relative as f64)/(100 as f64))) as usize];
        } else if tt == "mean"{
            thresh = mean_vecU16_u16(& work_on)
        } else {
            thresh = median(&work_on)
        }
        thresh = ((thresh as f64) * ((relative as f64)/(100 as f64))) as u16;



        thresh

    }

    pub fn get_norm_vec(&self, node: bool, absolute_thresh: &u16) -> Vec<u16>{
        let mut work_on: Vec<u16>;
        if node{
            work_on = self.get_node_cov_mean();

        } else {
            work_on = self.coverage.clone();
        }
        let mut ww: Vec<u16> = Vec::new();
        for x in work_on.iter(){
            ww.push(((*x as f64)/(*absolute_thresh as f64)) as u16)
        }
        ww
    }

    pub fn get_bit_vec(&self, node: bool, absolute_thresh: &u16) -> BitVec{
        let mut work_on: Vec<u16>;
        if node{
            work_on = self.get_node_cov_mean();

        } else {
            work_on = self.coverage.clone();
        }
        let mut bv: BitVec = BitVec::new();
        for x in work_on.iter(){
            if x >= absolute_thresh{
                bv.push(true)
            } else {
                bv.push(false)
            }
        }
        bv
    }

    pub fn get_node_cov_mean(&self) -> Vec<u16>{
        let mut node_id = self.node[0];
        let mut node_mean: Vec<u16> = Vec::new();
        let mut result: Vec<u16> = Vec::new();
        println!("{}", self.coverage.len());
        for x in 0..self.coverage.len() {
            if self.node[x] != node_id {
                result.push(mean_vecU16_u16(&node_mean));

                node_id = self.node[x];
                node_mean = vec![self.coverage[x] as u16];
            } else {
                node_mean.push(self.coverage[x] as u16)
            }
        }
        result
    }




}

pub fn remove_zero(vecc: & mut Vec<u16>){
    vecc.retain(|&x| x != 0);
}




