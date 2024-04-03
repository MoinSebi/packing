use std::process;
use clap::ArgMatches;
use log::info;
use packing_lib::convert::convert_helper::Method;
use packing_lib::convert::helper::{make_header, vec_u16_to_u8};
use packing_lib::core::core::PackCompact;
use packing_lib::core::reader::read_input;
use packing_lib::core::writer::writer_compress_zlib;


/// How to compress a pack file
pub fn compress_main(matches: &ArgMatches) {
    let (mut pc, index_present) = read_input(matches);

    // If name is set as argument, replace filename
    if matches.is_present("name") {
        pc.name = matches.value_of("name").unwrap().to_string();
    }

    // Check if there is data
    if pc.coverage.is_empty() {
        info!("There is a problem with the input files. Run 'packing info' on your file.");
        info!("[-h, --help] for help information");
        process::exit(0x0100);
    } else {
        info!("Name is {}", pc.name)
    }


    let num_entries = pc.coverage.len();
    let mut buffer = PackCompact::make_header(
        true,
        false,
        Method::Nothing,
        0.0,
        0.0,
        0.0,
        num_entries as u32,
        &pc.name,
    );


    buffer.extend(vec_u16_to_u8(&pc.coverage));

    println!("{}", buffer.len());
    writer_compress_zlib(&buffer, matches.value_of("out").unwrap());

}