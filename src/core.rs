use crate::helper::{binary2u8, vec_u16_u82, transform_u32_to_array_of_u8, u8_u322, mean_vec_u16};
use crate::reader::get_file_as_byte_vec;


pub struct PackCompact {
    pub node: Vec<u32>,
    pub coverage: Vec<u32>,
    pub coverage_normalized: Vec<f32>
}

impl PackCompact {
    /// Creating empty PackCompact struct
    pub fn new() -> Self {
        Self {
            node: Vec::new(),
            coverage: Vec::new(),
            coverage_normalized: Vec::new()
        }
    }

    /// Read from vg pack file
    pub fn read_complete(&mut self, filename: &str) {
        let buffer = get_file_as_byte_vec(filename);
        let chunks = buffer.chunks(4);
        for (i, x) in chunks.into_iter().enumerate() {
            if i % 2 == 0 {
                self.node.push(u8_u322(x))
            } else {
                self.coverage.push(u8_u322(x))
            }
        }
    }


    pub fn node2byte_thresh(&self, thresh: &u16) -> Vec<bool>{
        let mut node_id = 1;
        let mut node_mean: Vec<u16> = Vec::new();
        let mut result: Vec<bool> = Vec::new();
        for x in 0..self.coverage.len(){
            if self.node[x] != node_id {
                let mm = mean_vec_u16(&node_mean);
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
        for x in 0..self.coverage.len(){
            if self.node[x]  != node_id {
                result.push(mean_vec_u16(&node_mean));

                node_id = self.node[x] ;
                node_mean = vec![self.coverage[x] as u16];
            } else {
                node_mean.push(self.coverage[x] as u16)
            }
        }
        result

    }


    /// Coverage vector to byte vector
    /// Storing only u16 for max coverage
    pub fn coverage2byte(&self) -> Vec<u8> {
        let h = vec_u16_u82(&self.coverage);
        h
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

    /// Compress file
    pub fn compress_all(&self) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len(){
            buf.extend(transform_u32_to_array_of_u8(self.node[x]));
            buf.extend(transform_u32_to_array_of_u8(self.coverage[x]));
        }
        //println!("1 {}", buf.len());

        buf
    }

    /// Only nodes
    pub fn compress_only_node(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len() {
            buf.extend(transform_u32_to_array_of_u8(self.node[x]));
        }
        //println!("1 {}", buf.len());

        buf
    }

    /// Only coverage
    pub fn compress_only_coverage(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len() {
            buf.extend(transform_u32_to_array_of_u8(self.coverage[x]));
        }
        //println!("1 {}", buf.len());

        buf
    }

    #[allow(dead_code)]
    // This might be overkill
    pub fn compress_smart(&self) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        let mut node: &u32 = &0;
        let mut repeats: u32 = 0;
        let mut cov: Vec<u32> = Vec::new();

        for x in 0..self.coverage.len(){
           if (self.node[x] != *node) & (repeats != 0){
               buf.extend(transform_u32_to_array_of_u8(node.clone()));
               buf.extend(transform_u32_to_array_of_u8(repeats));
               buf.extend(vec_u16_u82(&cov));
               node = &self.node[x];
               repeats = 1;
               cov = vec![self.coverage[x]];

           }
            else {
                repeats += 1;
                cov.push(self.coverage[x].clone());
                node = &self.node[x];
            }
        }
        buf.extend(transform_u32_to_array_of_u8(node.clone()));
        buf.extend(transform_u32_to_array_of_u8(repeats));
        buf.extend(vec_u16_u82(&cov));
        println!("2 {}", buf.len());
        buf
    }



}




