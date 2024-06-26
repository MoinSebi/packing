use crate::index::index_wrapper::make_index;
use crate::info;
use clap::ArgMatches;
use log::warn;
use packing_lib::core::core::PackCompact;
use packing_lib::core::writer::writer_compress_zlib;
use std::path::Path;

pub fn index_main(matches: &ArgMatches) {
    if matches.is_present("gfa") {
        let j = matches.value_of("gfa").unwrap();
        if Path::new(matches.value_of("gfa").unwrap()).exists() {
            let o = matches.value_of("output").unwrap();
            let buf = make_index(j);
            writer_compress_zlib(&buf, o);
        } else {
            warn!("No file found");
            //process::exit(0x0100);
        }
    } else if matches.is_present("pack") {
        let o = matches.value_of("output").unwrap();
        let p = PackCompact::parse_pack(matches.value_of("pack").unwrap());
        let buf = p.node_index2buffer();
        writer_compress_zlib(&buf, o);
    } else {
        info!("No input")
    }
    //process::exit(0x0100);
}
