use std::fs::File;
use std::io::{Write, BufWriter};
use crate::helper::{zstd_encode};
use crate::core::PackCompact;



#[allow(dead_code)]
/// Just writing bytes to a file
pub fn writer_compress(buf: &Vec<u8>, filename: &str){
    let mut file = File::create(filename).expect("Not able to write ");
    file.write_all(&buf).expect("Not able to write ");
}



/// Just writing bytes to a file
pub fn writer_compress_zlib(buf: &Vec<u8>, filename: &str){
    let u = zstd_encode(buf);
    let mut file = File::create(filename).expect("Not able to write ");
    file.write_all(&u).expect("Not able to write ");
}


/// Writing normal pack file using the PackCompact structure
pub fn write_pack(pc: &PackCompact, filename: &str){
    let f = File::create([filename, "bin", "zst"].join(".")).expect("Unable to create file");
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





