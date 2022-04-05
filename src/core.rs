use log::info;
use crate::helper::{mean_vec_u16_u16, median, transform_u32_to_array_of_u8};

/// VG pack representation + additional information.
///
/// Is working with VG version 1.3 (maybe also earlier)
pub struct PackCompact {
    pub node: Vec<u32>,
    pub coverage: Vec<u16>,
    pub coverage_normalized: Vec<f32>,
    pub node_coverage: Vec<u16>
}

impl PackCompact {
    /// PackCompact constructor.
    ///
    /// No arguments.
    pub fn new() -> Self {
        Self {
            node: Vec::new(),
            coverage: Vec::new(),
            coverage_normalized: Vec::new(),
            node_coverage: Vec::new()
        }
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

    pub fn get_real_threshold(& mut self, node: bool, all: bool, relative: u16, tt: &str) -> u16{
        let mut work_on: Vec<u16>;
        if node{
            work_on = self.get_node_cov_mean();
            self.node_coverage = work_on.clone();
            self.coverage = Vec::new();

        } else {
            work_on = self.coverage.clone();
        }

         if all{
             remove_zero(& mut work_on)
         } else {
             remove_zero(& mut work_on)
         }
        let mut thresh;
        if tt != "nothing"{
            work_on.sort();
            thresh = work_on[(((work_on.len() as f64)/(100 as f64)) * ((relative as f64)/(100 as f64))).round() as usize];
        } else if tt == "mean"{
            thresh = mean_vec_u16_u16(& work_on);
            info!("Mean is {}", thresh);
        } else {
            thresh = median(&work_on);
            info!("Median is {}", thresh);
        }
        thresh = ((thresh as f64) * ((relative as f64)/(100 as f64))).round() as u16;
        info!("Working threshold is {}", thresh);



        thresh

    }



    pub fn get_node_cov_mean(&self) -> Vec<u16>{
        let mut node_id = self.node[0];
        let mut node_mean: Vec<u16> = Vec::new();
        let mut result: Vec<u16> = Vec::new();
        println!("{}", self.coverage.len());
        for x in 0..self.coverage.len() {
            if self.node[x] != node_id {
                result.push(mean_vec_u16_u16(&node_mean));

                node_id = self.node[x];
                node_mean = vec![self.coverage[x] as u16];
            } else {
                node_mean.push(self.coverage[x] as u16)
            }
        }
        result.push(mean_vec_u16_u16(&node_mean));
        result
    }




}

pub fn remove_zero(vecc: & mut Vec<u16>){
    vecc.retain(|&x| x != 0);
}




