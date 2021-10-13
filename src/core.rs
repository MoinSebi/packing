use crate::helper::{vec_u16_u8, binary2u8};

pub struct Pack {
    pub header: String,
    pub name: String,
    pub node: Vec<u32>,
    pub seq: Vec<u32>,
    pub cov: Vec<u16>,
    pub offset: Vec<u32>
}

impl Pack {
    pub fn new() -> Self{
        let k = Vec::new();
        let k2 = Vec::new();
        let k3 = Vec::new();
        let k4 = Vec::new();
        let kk: String = "".to_string();
        let name2 = "".to_string();

        Self{
            node: k,
            seq: k2,
            cov: k3,
            offset: k4,
            name: kk,
            header: name2,
        }
    }
    pub fn cov2u8(&self) -> Vec<u8>{
        let h = vec_u16_u8(&self.cov);
        h
    }

    pub fn cov2u8_thres(&self, thresh: &u16) -> Vec<u8>{
        let mut j: Vec<bool> = Vec::new();
        for x in self.cov.iter(){
            if x >= thresh{
                j.push(true)
            } else {
                j.push(false);
            }
        }
        let h = binary2u8(j);
        h
    }


}



pub struct read_in{
    pub ty: String,
    pub name: String,
    pub cc: Vec<u16>,
}


pub struct read_in2{
    pub ty: String,
    pub name: String,
    pub cc: Vec<bool>,
}

