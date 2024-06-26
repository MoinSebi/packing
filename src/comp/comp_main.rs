use clap::ArgMatches;
use log::info;
use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::unpack_zstd_to_byte;
use packing_lib::normalize::convert_helper::Method;

pub fn comp_main(matches: &ArgMatches) {
    info!("Compare main");
    let filename1 = matches.value_of("pack compressed1").unwrap();
    let filename2 = matches.value_of("pack compressed2").unwrap();
    let g: Vec<u8> = unpack_zstd_to_byte(filename1);
    let meta1 = PackCompact::get_meta(&g);

    let g: Vec<u8> = unpack_zstd_to_byte(filename2);
    let meta2 = PackCompact::get_meta(&g);
    compare_meta(&meta1, &meta2);
}

fn compare_meta(
    meta1: &(
        bool,
        bool,
        DataType,
        Method,
        f32,
        f32,
        f32,
        u32,
        u32,
        String,
    ),
    meta2: &(
        bool,
        bool,
        DataType,
        Method,
        f32,
        f32,
        f32,
        u32,
        u32,
        String,
    ),
) {
    let mut all_good = true;
    if meta1.0 != meta2.0 {
        all_good = false;
        info!("Entry type is different");
    }
    if meta1.1 != meta2.1 {
        all_good = false;

        info!("Include all not the same");
    }

    if meta1.2 != meta2.2 {
        all_good = false;

        info!("Data type is different");
    }
    if meta1.3 != meta2.3 {
        all_good = false;

        info!("Method is different");
    }
    if meta1.4 != meta2.4 {
        all_good = false;

        info!("Relative threshold is different");
    }
    if meta1.5 != meta2.5 {
        all_good = false;

        info!("Standard deviation is different");
    }
    if meta1.6 != meta2.6 {
        all_good = false;

        info!("Real threshold is different");
    }
    if meta1.7 != meta2.7 {
        all_good = false;

        info!("Bytes is different");
    }
    if meta1.8 != meta2.8 {
        all_good = false;

        info!("Entries is different");
    }
    if all_good {
        info!("Meta data is the same");
    }
}
