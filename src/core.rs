use crate::helper::{vec_u16_u8, binary2u8, vec_u16_u82, transform_u32_to_array_of_u8, u8_u322};
use crate::reader::get_file_as_byte_vec;


pub struct PackCompact {
    pub node: Vec<u32>,
    pub coverage: Vec<u32>
}

impl PackCompact {

    /// Creating empty PackCompact struct
    pub fn new()->Self{
        let node = Vec::new();
        let cov = Vec::new();

        Self{
            node: node,
            coverage: cov,
        }
    }

    /// Read from vg pack file
    pub fn read_complete(& mut self, filename: &str){

        let buffer = get_file_as_byte_vec(filename);
        let chunks = buffer.chunks(4 );
        for (i, x) in chunks.into_iter().enumerate(){
            if i%2 == 0{
                self.node.push(u8_u322(x))
            }
            else{
                self.coverage.push(u8_u322(x))
            }
        }
    }



    /// Coverage vector to byte vector
    pub fn coverage2byte(&self) -> Vec<u8>{
        let h = vec_u16_u82(&self.coverage);
        h
    }

    pub fn coverage2byte_thresh_bit(&self, thresh: &u16) -> Vec<u8>{
        let mut j: Vec<bool> = Vec::new();
        for x in self.coverage.iter(){
            if *x as u16 >= *thresh{
                j.push(true)
            } else {
                j.push(false);
            }
        }
        let h = binary2u8(&j);
        h
    }

    pub fn compress(&self) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..self.coverage.len(){
            buf.extend(transform_u32_to_array_of_u8(self.node[x]));
            buf.extend(transform_u32_to_array_of_u8(self.coverage[x]));
        }
        println!("1 {}", buf.len());

        buf
    }

    // This might be overkill
    pub fn compress2(&self) -> Vec<u8>{
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




