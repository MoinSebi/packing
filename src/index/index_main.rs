use gfaR_wrapper::{NGfa};
use crate::helper::transform_u32_to_array_of_u8;
use crate::writer::writer_compress_zlib;


/// Read GFA and get nodes + sequences
/// Same order than VG --> sort(node, sequence)
pub fn make_index(filename: &str) -> Vec<u8>{

    let mut graph = NGfa::new();
    graph.from_graph(filename);

    let mut nodes: Vec<u32> = graph.nodes.keys().cloned().collect();
    println!("nn {}", nodes.len());
    nodes.sort_by(|a, b| a.partial_cmp(b).unwrap());


    let mut buf = Vec::new();
    let mut count = 0;
    for x in nodes.iter() {
        for _y in 0..graph.nodes.get(x).unwrap().len {
            count += 1;
            buf.extend(transform_u32_to_array_of_u8(x.clone()));
        }

    }
    println!("count {}", count);
    return buf

}


