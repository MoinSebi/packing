use std::fs::File;
use std::io::{Write, BufWriter};
use crate::core::core::PackCompact;


#[allow(dead_code)]
/// Just writing bytes to a file
pub fn writer_compress(buf: &Vec<u8>, filename: &str){
    let mut file = File::create(filename).expect("Not able to write ");
    file.write_all(&buf).expect("Not able to write ");
}

/// Helper function for zstd encoder
/// /// https://docs.rs/zstd/0.1.9/zstd/struct.Endcoder.html
pub fn zstd_encode(v: &Vec<u8>) -> Vec<u8>{
    let mut e = zstd::Encoder::new(Vec::new(), 0).unwrap();
    e.write_all(v).expect("Not working");
    let com = e.finish().unwrap();
    return com
}


/// Just writing bytes to a file
pub fn writer_compress_zlib(buf: &Vec<u8>, filename: &str){
    let u = zstd_encode(buf);
    let mut file = File::create(filename).expect("Not able to write ");
    file.write_all(&u).expect("Not able to write ");
}


/// Writing normal pack file using the PackCompact structure
pub fn write_pack(pc: &PackCompact, filename: &str){
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "{}\t{}\t{}\t{}\n", "seq.pos", "node.id", "node.offset", "coverage").expect("Can not write file");

    let mut node = 0;
    for x in 0..pc.coverage.len(){
        if x == 0{
            write!(f, "{}\t{}\t{}\t{}\n", x, pc.node[x], node, pc.coverage[x]).expect("Can not write file");
        }else {
            if pc.node[x] == pc.node[x - 1] {
                node += 1;
                write!(f, "{}\t{}\t{}\t{}\n", x, pc.node[x], node, pc.coverage[x]).expect("Can not write file");
            } else {
                node = 0;
                write!(f, "{}\t{}\t{}\t{}\n", x, pc.node[x], node, pc.coverage[x]).expect("Can not write file");
            }
        }



    }
}

impl PackCompact{
    pub fn write_pack(&self, filename: &str){
        let f = File::create(filename).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        write!(f, "{}\t{}\t{}\t{}\n", "seq.pos", "node.id", "node.offset", "coverage").expect("Can not write file");

        let mut node = 0;
        for x in 0..self.coverage.len(){
            if x == 0{
                write!(f, "{}\t{}\t{}\t{}\n", x, self.node[x], node, self.coverage[x]).expect("Can not write file");
            }else {
                if self.node[x] == self.node[x - 1] {
                    node += 1;
                    write!(f, "{}\t{}\t{}\t{}\n", x, self.node[x], node, self.coverage[x]).expect("Can not write file");
                } else {
                    node = 0;
                    write!(f, "{}\t{}\t{}\t{}\n", x, self.node[x], node, self.coverage[x]).expect("Can not write file");
                }
            }



        }
    }


}





