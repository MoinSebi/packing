use crate::info::info_helper::{info_compressed, info_index};
use clap::ArgMatches;

pub fn info_main(matches: &ArgMatches) {
    if matches.is_present("index") | (matches.is_present("pack compressed")) {
        if matches.is_present("index") {
            info_index(matches.value_of("index").unwrap());
        } else {
            info_compressed(matches.value_of("pack compressed").unwrap());
        }
    }
}
