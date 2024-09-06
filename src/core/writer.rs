use crate::core::core::{DataType, PackCompact};
use crate::normalize::convert_helper::Method;
use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(dead_code)]
/// Just writing bytes to a file
pub fn writer_compress(buf: &[u8], filename: &str) {
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
        writeln!(
            f,
            "{}\t{}\t{}\t{}",
            0, self.node_index[0], node, self.coverage[0]
        )
        .expect("Can not write file");
        for x in 1..self.coverage.len() {
            if self.node_index[x] == self.node_index[x - 1] {
                node += 1;
                writeln!(
                    f,
                    "{}\t{}\t{}\t{}",
                    x, self.node_index[x], node, self.coverage[x]
                )
                .expect("Can not write file");
            } else {
                node = 0;
                writeln!(
                    f,
                    "{}\t{}\t{}\t{}",
                    x, self.node_index[x], node, self.coverage[x]
                )
                .expect("Can not write file");
            }
        }
    }

    pub fn write_pack_f32(&self, filename: &str) {
        let f = File::create(filename).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        writeln!(f, "seq.pos\tnode.id\tnode.offset\tcoverage").expect("Can not write file");

        let mut node = 0;
        writeln!(
            f,
            "{}\t{}\t{}\t{}",
            0, self.node_index[0], node, self.normalized_coverage[0]
        )
        .expect("Can not write file");
        for x in 1..self.coverage.len() {
            if self.node_index[x] == self.node_index[x - 1] {
                node += 1;
                writeln!(
                    f,
                    "{}\t{}\t{}\t{}",
                    x, self.node_index[x], node, self.normalized_coverage[x]
                )
                .expect("Can not write file");
            } else {
                node = 0;
                writeln!(
                    f,
                    "{}\t{}\t{}\t{}",
                    x, self.node_index[x], node, self.normalized_coverage[x]
                )
                .expect("Can not write file");
            }
        }
    }

    /// Construct a header (data) from a PackCompact
    ///
    /// This is the order:
    /// - sequence/node
    /// - DataType (bit, u16, f32)
    /// - method
    /// - relative
    /// - std
    /// - real thresh
    /// - length
    /// - name
    pub fn file_header(
        want_sequence: bool,
        include_all: bool,
        is_binary: DataType,
        method: Method,
        relative: f32,
        std: f32,
        thresh: f32,
        length: u32,
        name: &str,
    ) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![53, 56];

        // Is node?

        buffer.push(if want_sequence { 1 } else { 0 });
        buffer.push(if include_all { 1 } else { 0 });
        // Is binary?
        buffer.push(is_binary.to_u8());

        match method {
            Method::Nothing => buffer.push(0),
            Method::Mean => buffer.push(1),
            Method::Median => buffer.push(2),
            Method::Percentile => buffer.push(3),
            Method::Absolute => buffer.push(4),
            Method::Compress => buffer.push(5),
        }

        // Relative threshold
        let mut buff2 = vec![0; 4];
        BigEndian::write_f32(&mut buff2, relative);
        buffer.extend(buff2);

        let mut buff2 = vec![0; 4];
        BigEndian::write_f32(&mut buff2, std);
        buffer.extend(buff2);

        // Real Threshold
        let mut buff2 = vec![0; 4];
        BigEndian::write_f32(&mut buff2, thresh);
        buffer.extend(buff2);

        // Length of the vector
        let mut buff2 = vec![0; 4];
        BigEndian::write_u32(&mut buff2, length);
        buffer.extend(buff2);

        // Name
        let char_vec: Vec<char> = name.chars().collect();
        for c in char_vec.iter() {
            buffer.push(*c as u8);
        }
        // Add space
        for _x in 0..(64 - char_vec.len()) {
            buffer.push(32);
        }
        buffer
    }
}
