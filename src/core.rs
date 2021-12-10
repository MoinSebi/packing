use crate::helper::{binary2u8, vec_u16_u82, transform_u32_to_array_of_u8, u8_u322, mean_vec_u16, mean_vec_u32, median, mean_vec_f32, vec_f32_u82};
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

    // Create

    /// Read from vg pack file
    pub fn read_complete(&mut self, filename: &str) {
        print!("Filename {}\n", filename);
        let buffer = get_file_as_byte_vec(filename);
        print!("Size1 {}", buffer.len());
        let chunks = buffer.chunks(4);
        for (i, x) in chunks.into_iter().enumerate() {
            if i % 2 == 0 {
                self.node.push(u8_u322(x))
            } else {
                self.coverage.push(u8_u322(x))
            }
        }
    }



    // Modification

    pub fn normalize_wrapper(&mut self, kind: &str){
        let mut values = Vec::new();
        if kind == "mean"{
            for x in self.coverage.iter(){
                values.push(x.clone());
            }
            let h = mean_vec_u32(&values);

            for x in self.coverage.iter(){
                self.coverage_normalized.push(*x as f32/h as f32);
            }
        } else if kind == "sum"{
            let mut sum = 0;
            for x in self.coverage.iter(){
                sum += x;
            }

            for x in self.coverage.iter(){
                self.coverage_normalized.push(*x as f32/sum as f32);
            }
        } else {
            for x in self.coverage.iter(){
                values.push(x.clone());
            }
            let h = median(&values);

            for x in self.coverage.iter(){
                self.coverage_normalized.push(*x as f32/h as f32);
            }
            println!("{} {}", self.coverage.len(), self.coverage_normalized.len());

        }
    }

    #[allow(dead_code)]
    /// Normalize coverages by mean
    pub fn normalize_covered_mean(&mut self){
        let mut values = Vec::new();
        for x in self.coverage.iter(){
            values.push(x.clone());
        }
        let h = mean_vec_u32(&values);

        for x in self.coverage.iter(){
            self.coverage_normalized.push(*x as f32/h as f32);
        }
        println!("{} {}", self.coverage.len(), self.coverage_normalized.len());

    }


    /// Normalize coverages by meadian
    pub fn normalize_covered_median(&mut self) -> u32{
        let mut values = Vec::new();
        for x in self.coverage.iter(){
            values.push(x.clone());
        }
        let h = median(&values);

        for x in self.coverage.iter(){
            self.coverage_normalized.push(*x as f32/h as f32);
        }
        println!("{} {}", self.coverage.len(), self.coverage_normalized.len());
        return h;


    }


    /// Normalize coverages by total sum
    pub fn normalize_covered_sum(&mut self){
        let mut sum = 0;
        for x in self.coverage.iter(){
            sum += x;
        }

        for x in self.coverage.iter(){
            self.coverage_normalized.push(*x as f32/sum as f32);
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

    // NORMALIZED


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

    pub fn coverage2byte_normalized(&self) -> Vec<u8> {
        let h = vec_f32_u82(&self.coverage_normalized);
        h
    }


    pub fn cov2byte_thresh_normalized(&self, thresh: &f32) -> Vec<u8>{
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

    /// Compress total coverage to binary represenation
    pub fn compress_all(&self) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len(){
            buf.extend(transform_u32_to_array_of_u8(self.node[x]));
            buf.extend(transform_u32_to_array_of_u8(self.coverage[x]));
        }
        //println!("1 {}", buf.len());

        buf
    }

    /// Compress only the nodes (index)
    pub fn compress_only_node(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len() {
            buf.extend(transform_u32_to_array_of_u8(self.node[x]));
        }
        //println!("1 {}", buf.len());

        buf
    }

    /// Compress ony the coverage
    pub fn compress_only_coverage(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len() {
            buf.extend(transform_u32_to_array_of_u8(self.coverage[x]));
        }
        //println!("1 {}", buf.len());

        buf
    }


    #[allow(dead_code)]
    // This might be overkill - keep it for later
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




