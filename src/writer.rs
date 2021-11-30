use std::fs::File;
use std::io::{Write, BufWriter};
use crate::helper::{transform_u16_to_array_of_u8, transform_u32_to_array_of_u8, zstd_encode};
use crate::core::PackCompact;


/// Writer for u8
/// Output: 2 bytes (identifier), 4 byte (länge), 1(vec<u16>)
/// To add, threshold yes no -> bit
/// if not -> number threshold number (0 if nothing)
pub fn write_file(name: &str, vecc: &Vec<u8>, tresh: u16, out: &str, b: bool){
    let s2:Vec<&str> = name.split("/").collect();

    let s = s2.last().unwrap().clone();
    // this is the identifier
    let mut buff: Vec<u8> = vec![1,1];
    if b{
        buff.push(1);
    } else {
        buff.push(0);
    }
    println!("{}", vecc.len());
    println!("{:?}", transform_u32_to_array_of_u8(vecc.len() as u32));
    // Length of the vector
    buff.extend(transform_u32_to_array_of_u8(vecc.len() as u32));
    // Add threshold
    buff.extend(transform_u16_to_array_of_u8(tresh));


    // Name
    let char_vec: Vec<char> = s.chars().collect();



    for c in char_vec.iter() {
        buff.push(c.clone() as u8);
    }

    for _x in 0..(64 - char_vec.len()){
        buff.push(0);
    }


    for x in vecc.iter(){
        buff.push(x.clone());
    }
    let buf2 = zstd_encode(&buff);
    let mut file = File::create([out, "bin"].join(".")).expect("Not able to write ");
    file.write_all(&buf2).expect("Not able to write ");

}

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

#[cfg(test)]
mod write {
    use crate::vg_parser::{parse_smart};
    use crate::writer::{write_pack, writer_compress, write_file, writer_compress_zlib};
    use crate::reader::wrapper_meta;
    use crate::core::PackCompact;
    use crate::helper::{vec_u16_u8, binary2u8};

    #[test]
    fn pack_pack() {
        let k = parse_smart("testing/9986.100k.txt");
        write_pack(&k, "testing/write_pack.bin");
    }
    #[test]
    fn pack_meta() {
        let p = parse_smart("testing/9986.100k.txt");
        let buf = p.compress_only_node();
        writer_compress_zlib(&buf, "testing/meta_test.bin");
    }

    #[test]
    fn pack_cov() {
        let p = parse_smart("testing/9986.100k.txt");
        let buf = p.compress_only_node();
        writer_compress(&buf, "testing/node_test.bin");
    }

    #[test]
    fn bin_cov() {
        let p = wrapper_meta("testing/meta_test.bin", "testing/coverage_test.bin");
        let buf = p.compress_only_coverage();
        writer_compress(&buf, "testing/coverage_test.bin");
    }


    #[test]
    fn pack_all() {
        let p = parse_smart("testing/9986.100k.txt");
        let buf = p.compress_all();
        writer_compress(&buf, "testing/all_test.bin");

    }

    #[test]
    fn bin_bin_pack() {
        let mut p = PackCompact::new();
        p.read_complete("testing/all_test.bin");
        let buf = p.compress_all();
        writer_compress(&buf, "testing/all_test2.bin");
        write_pack(&p, "testing/9986.alt.txt");
    }

    #[test]
    fn bin_smart_node() {
        let mut p = PackCompact::new();
        p.read_complete("testing/all_test.bin");
        let mean_node_out = p.coverage2byte();
        write_file("test1", &mean_node_out, 0, "testing/smart_cov.bin", false);
    }


    #[test]
    fn bin_smart_coverage() {
        let mut p = PackCompact::new();
        p.read_complete("testing/all_test.bin");
        let mean_node_out = vec_u16_u8(&p.node2byte());
        write_file("test1", &mean_node_out, 0, "testing/smart_node.bin", false);
    }


    #[test]
    fn bin_smart_node_thresh() {
        let mut p = PackCompact::new();
        p.read_complete("testing/all_test.bin");
        let mean_node_out = binary2u8(&p.node2byte_thresh(&10));
        write_file("test1", &mean_node_out, 0, "testing/smart_node2.bin", false);
    }


    #[test]
    fn bin_smart_coverage_thresh() {
        let mut p = PackCompact::new();
        p.read_complete("testing/all_test.bin");
        let mean_node_out = p.coverage2byte_thresh_bit(&10);
        write_file("test1", &mean_node_out, 0, "testing/smart_cov2.bin", false);
    }

    #[test]
    fn bin_smart_coverage_thresh2() {
        let p = parse_smart("testing/9986.100k.txt");
        let buf = p.compress_all();
        writer_compress_zlib(&buf, "testing/all_test.zst");

        let mut  p2 = PackCompact::new();
        p2.read_complete("testing/all_test.zst");
        write_pack(&p2, "testing/jo.reverse.txt");
    }


}





