use crate::core::core::PackCompact;
use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(dead_code)]
/// Just writing bytes to a file
pub fn writer_compress(buf: &Vec<u8>, filename: &str) {
    let mut file = File::create(filename).expect("Not able to write ");
    file.write_all(buf).expect("Not able to write ");
}

/// Helper function for zstd encoder
/// /// https://docs.rs/zstd/0.1.9/zstd/struct.Endcoder.html
pub fn zstd_encode(v: &Vec<u8>) -> Vec<u8> {
    let mut e = zstd::Encoder::new(Vec::new(), 0).unwrap();
    e.write_all(v).expect("Not working");

    e.finish().unwrap()
}

/// Just writing bytes to a file
pub fn writer_compress_zlib(buf: &Vec<u8>, filename: &str) {
    let u = zstd_encode(buf);
    let mut file = File::create(filename).expect("Not able to write ");
    file.write_all(&u).expect("Not able to write ");
}

impl PackCompact {
    pub fn write_pack(&self, filename: &str) {
        let f = File::create(filename).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        writeln!(f, "seq.pos\tnode.id\tnode.offset\tcoverage").expect("Can not write file");

        let mut node = 0;
        for x in 0..self.coverage.len() {
            if x == 0 {
                writeln!(f, "{}\t{}\t{}\t{}", x, self.node[x], node, self.coverage[x])
                    .expect("Can not write file");
            } else if self.node[x] == self.node[x - 1] {
                node += 1;
                writeln!(f, "{}\t{}\t{}\t{}", x, self.node[x], node, self.coverage[x])
                    .expect("Can not write file");
            } else {
                node = 0;
                writeln!(f, "{}\t{}\t{}\t{}", x, self.node[x], node, self.coverage[x])
                    .expect("Can not write file");
            }
        }
    }
}
