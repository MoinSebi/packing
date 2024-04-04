use clap::ArgMatches;
use log::info;


use packing_lib::core::core::PackCompact;
use packing_lib::core::reader::unpack_zstd_to_byte;
use packing_lib::core::writer::writer_compress_zlib;

pub fn rename_main1(matches: &ArgMatches) {
    info!("Renaming");
    eprintln!("Renaming matches {:?}", matches.is_present("input"));
    let filename = matches.value_of("input").unwrap();
    let new_name = matches.value_of("name").unwrap();
    let out = matches.value_of("output").unwrap();
    let g: Vec<u8> = unpack_zstd_to_byte(filename);
    let meta = PackCompact::get_meta(&g);
    let _p = PackCompact::read_wrapper(filename);

    let bin = meta.1;

    let mut header = PackCompact::file_header(
        meta.0, bin, meta.2, meta.3, meta.4, meta.5, meta.6, new_name,
    );
    header.extend(&g[77..]);
    writer_compress_zlib(&header, out);
}
