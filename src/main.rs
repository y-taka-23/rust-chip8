mod buzzer;
mod chip8;
mod display;
mod keyboard;
mod memory;

use chip8::{Chip8, Flags};

use chrono::Local;
use clap::{app_from_crate, arg};
use fern::Dispatch;
use iced::{Application, Color, Settings};
use log::LevelFilter;
use std::fs::File;
use std::io::{stderr, Read};

fn main() {
    let matches = app_from_crate!()
        .arg(arg!([FILE] "File of the CHIP-8 ROM").required(true))
        .arg(arg!(--clock [INT] "Change the clock speed (1-500 Hz)").default_value("500"))
        .arg(
            arg!(--color [STRING] "Select the display color (white/green/amber)")
                .default_value("white"),
        )
        .arg(arg!(--verbose "Show the detailed execution trace"))
        .get_matches();

    let file_name = matches.value_of("FILE").unwrap();
    let mut file = File::open(file_name).unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    let clock_speed: u64 = matches.value_of("clock").unwrap().parse().unwrap();
    if 500 < clock_speed {
        panic!("Unsupported clock speed: {} Hz", clock_speed);
    }

    let color = matches.value_of("color").unwrap();
    let display_color = match color {
        "white" => Color::new(0.95, 0.95, 0.95, 1.0),
        "green" => Color::new(0.0, 0.95, 0.0, 1.0),
        "amber" => Color::new(0.95, 0.75, 0.0, 1.0),
        _ => panic!("Unsupported display color: {}", color),
    };

    let is_verbose = matches.is_present("verbose");
    init_logger(is_verbose);

    let flags = Flags {
        rom,
        clock_speed,
        display_color,
    };
    let mut settings = Settings::with_flags(flags);
    settings.window.size = (display::WIDTH as u32, display::HEIGHT as u32);
    Chip8::run(settings).unwrap()
}

fn init_logger(is_verbose: bool) {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Error)
        .level_for(
            "chip8",
            if is_verbose {
                LevelFilter::Trace
            } else {
                LevelFilter::Error
            },
        )
        .chain(stderr())
        .apply()
        .unwrap();
}
