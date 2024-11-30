use crate::info::info_helper::{info_compressed, info_index};
use clap::ArgMatches;
use log::info;

pub fn info_main(matches: &ArgMatches) {
    info!("Running 'packing info'");
    if matches.is_present("index") | (matches.is_present("pack compressed")) {
        if matches.is_present("index") {
            info!("Index info\n");
            info_index(matches.value_of("index").unwrap());
        } else {
            info!("Pack info\n");
            info_compressed(matches.value_of("pack compressed").unwrap());
        }
    }
}
