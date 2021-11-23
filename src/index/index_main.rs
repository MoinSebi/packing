use gfaR_wrapper::{GraphWrapper, NGfa};
use std::fs::File;
use std::fs;
use std::io::BufWriter;
use crate::helper::transform_u32_to_array_of_u8;
use crate::writer::writer_compress_zlib;

pub struct pack{
    pub node: u32,
    pub size: u32,
}



pub fn makeIndex(filename: &str, output: &str){
    let mut graph = NGfa::new();
    graph.from_graph("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa");
    let vec_data: Vec<pack> = Vec::new();

    let mut h: Vec<u32> = graph.nodes.keys().cloned().collect();

    h.sort_by(|a, b| a.partial_cmp(b).unwrap());


    let mut h2 = Vec::new();
    for x in h.iter() {
        for _y in 0..graph.nodes.get(x).unwrap().len {
            h2.extend(transform_u32_to_array_of_u8(x.clone()));
        }

    }
    writer_compress_zlib(&h2, output);

}

pub fn function(){
    println!("dsakjdsa");
}


mod index {
    use crate::index::index_main::makeIndex;

    #[test]
    fn pack_pack() {
        let k = makeIndex("holyshit32131920839021");

    }
}