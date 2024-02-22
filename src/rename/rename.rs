use crate::convert::convert_helper::{Method, OutputType};
use crate::convert::helper::make_header;
use crate::core::core::PackCompact;
use crate::core::reader::{get_meta, unpack_zstd_to_byte};
use crate::core::writer::writer_compress_zlib;
use clap::ArgMatches;
use log::info;

pub fn rename_main(matches: &ArgMatches) {
    info!("Renaming");
    let filename = matches.value_of("input").unwrap();
    let new_name = matches.value_of("name").unwrap();
    let out = matches.value_of("output").unwrap();
    let g: Vec<u8> = unpack_zstd_to_byte(filename);
    let meta = get_meta(&g);
    let _p = PackCompact::wrapp(filename);

    let nodes = if meta.0 {
        OutputType::Node
    } else {
        OutputType::Sequence
    };
    let bin = meta.1;
    let method = Method::from_u8(meta.2);

    let mut header = make_header(nodes, bin, method, meta.3, &meta.4, meta.6, new_name);
    header.extend(&g[77..]);
    writer_compress_zlib(&header, out);
}

pub fn rename(pc: &mut PackCompact, new_name: &str) {
    pc.name = new_name.to_string();
}
