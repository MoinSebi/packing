use gfaR_wrapper::{NGfa};
use crate::helper::transform_u32_to_array_of_u8;
use crate::writer::writer_compress_zlib;

pub fn make_index(filename: &str, output: &str){
    let mut graph = NGfa::new();
    graph.from_graph(filename);

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


#[cfg(test)]
mod index {
    use crate::index::index_main::make_index;

    #[test]
    fn pack_pack() {
        make_index("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa", "holyshit32131920839021");

    }
}