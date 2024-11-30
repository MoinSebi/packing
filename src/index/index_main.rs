use crate::index::index_wrapper::make_index;
use crate::info;
use clap::ArgMatches;
use log::warn;
use packing_lib::core::core::PackCompact;
use packing_lib::core::writer::writer_compress_zlib;
use std::path::Path;

pub fn index_main(matches: &ArgMatches) {
    info!("Running 'packing index'");
    if matches.is_present("gfa") {
        info!("Indexing GFA file");
        let j = matches.value_of("gfa").unwrap();
        if Path::new(matches.value_of("gfa").unwrap()).exists() {
            let output = matches.value_of("output").unwrap();
            let buf = make_index(j);
            writer_compress_zlib(&buf, output);
        } else {
            warn!("No file found");
            //process::exit(0x0100);
        }
    } else if matches.is_present("pack") {
        info!("Indexing pack file");
        let output_file = matches.value_of("output").unwrap();
        let pack = PackCompact::parse_pack(matches.value_of("pack").unwrap());
        let buffer = pack.node_index2buffer();
        writer_compress_zlib(&buffer, output_file);
    } else {
        info!("No input")
    }
    info!("Done");
}
