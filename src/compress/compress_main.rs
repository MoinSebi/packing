use clap::ArgMatches;
use log::info;
use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::{get_input_args, read_input2};
use packing_lib::core::writer::writer_compress_zlib;
use packing_lib::normalize::convert_helper::Method;
use packing_lib::normalize::helper::vec_u16_to_u8;
use std::process;

/// How to compress a pack file
pub fn compress_main(matches: &ArgMatches) {
    let input_pack = get_input_args(matches, "pack");
    let input_index = get_input_args(matches, "index");
    let input_pc = get_input_args(matches, "pc");

    let (mut pc, _index_present) = read_input2(&input_pack, &input_index, &input_pc);

    if pc.data_type != DataType::TypeU16 {
        info!("Your input is not a plain-text coverage file");
        process::exit(0x0100);
    }

    // If name is set as argument, replace filename
    if matches.is_present("name") {
        pc.name = matches.value_of("name").unwrap().to_string();
    }

    // Check if there is data
    if pc.coverage.is_empty() {
        info!("There is a problem with the input files. Run 'packing info' on your file.");
        info!("[-h, --help] for help information");
        process::exit(0x0100);
    }

    let num_entries = pc.coverage.len();
    let mut buffer = PackCompact::file_header(
        true,
        true,
        DataType::TypeU16,
        Method::Nothing,
        0.0,
        0.0,
        0.0,
        num_entries as u32,
        &pc.name,
    );

    buffer.extend(vec_u16_to_u8(&pc.coverage));

    writer_compress_zlib(&buffer, matches.value_of("output").unwrap());
}
