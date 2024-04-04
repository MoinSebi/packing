use clap::ArgMatches;
use log::info;
use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::read_input;
use packing_lib::normalize::helper::{
    mean, mean_vec_u16_f64, median, median_vec_u16_16, remove_zero, remove_zero_f32,
    remove_zero_new,
};
use std::fs::File;
use std::io::{self, Write};

pub fn stats_main(matches: &ArgMatches) {
    info!("Stats main");
    let (mut pc, index_present) = read_input(matches);
    let output = matches.value_of("output").unwrap_or("-");
    if output == "-" {
        stats_wrapper(&mut pc, index_present, &mut None);
    } else {
        let mut file = File::create(output).expect("Unable to create file");
        stats_wrapper(&mut pc, index_present, &mut Some(&mut file));
    }
}

fn write_to_file_or_stdout(file: &mut Option<&mut File>, content: &str) -> io::Result<()> {
    match file {
        Some(f) => {
            f.write_all(content.as_bytes())?;
        }
        None => {
            println!("{}", content);
        }
    }
    Ok(())
}

pub fn stats_wrapper(pc: &mut PackCompact, _index_present: bool, file2: &mut Option<&mut File>) {
    write_to_file_or_stdout(file2, &format!("Name: {}", pc.name)).expect("Can not write file");
    if pc.data_type == DataType::TypeBit {
        info!("Is bit!");
        write_to_file_or_stdout(
            file2,
            &format!(
                "Number presence entries {}",
                pc.bin_coverage.iter().filter(|x| **x).count()
            ),
        )
        .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!(
                "Number absence entries {}",
                pc.bin_coverage.iter().filter(|x| !**x).count()
            ),
        )
        .expect("Can not write file");
    } else if pc.data_type == DataType::TypeU16 {
        info!("Is U16");
        if !pc.is_sequence {
            let mut workon = pc.coverage.clone();
            write_to_file_or_stdout(file2, &format!("Average (with zeros) {}", mean(&workon)))
                .expect("Can not write file");
            write_to_file_or_stdout(
                file2,
                &format!("Median (with zeros) {}", median(&mut workon)),
            )
            .expect("Can not write file");
            remove_zero_new(&workon);
            write_to_file_or_stdout(
                file2,
                &format!("Average (without zeros) {}", mean_vec_u16_f64(&workon)),
            )
            .expect("Can not write file");
            write_to_file_or_stdout(
                file2,
                &format!("Median (without zeros) {}", median_vec_u16_16(&mut workon)),
            )
            .expect("Can not write file");
        } else {
            println!("dasjkdsa");
            let mut workon = pc.coverage.clone();
            write_to_file_or_stdout(file2, &format!("Average (with zeros) {}", mean(&workon)))
                .expect("Can not write file");
            write_to_file_or_stdout(
                file2,
                &format!("Median (with zeros) {}", median(&mut workon)),
            )
            .expect("Can not write file");
            remove_zero(&mut workon);
            write_to_file_or_stdout(file2, &format!("Average (without zeros) {}", mean(&workon)))
                .expect("Can not write file");
            write_to_file_or_stdout(
                file2,
                &format!("Median (without zeros) {}", median(&mut workon)),
            )
            .expect("Can not write file");
        }
    } else if !pc.is_sequence {
        let mut workon = pc.normalized_coverage.clone();
        write_to_file_or_stdout(file2, &format!("Average (with zeros) {}", mean(&workon)))
            .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!("Median (with zeros) {}", median(&mut workon)),
        )
        .expect("Can not write file");
        remove_zero_f32(&mut workon);
        write_to_file_or_stdout(file2, &format!("Average (without zeros) {}", mean(&workon)))
            .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!("Median (without zeros) {}", median(&mut workon)),
        )
        .expect("Can not write file");
    } else {
        let mut workon = pc.normalized_coverage.clone();
        write_to_file_or_stdout(file2, &format!("Average (with zeros) {}", mean(&workon)))
            .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!("Median (with zeros) {}", median(&mut workon)),
        )
        .expect("Can not write file");
        remove_zero_f32(&mut workon);
        write_to_file_or_stdout(file2, &format!("Average (without zeros) {}", mean(&workon)))
            .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!("Median (without zeros) {}", median(&mut workon)),
        )
        .expect("Can not write file");
    }
}
