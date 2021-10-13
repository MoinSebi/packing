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

