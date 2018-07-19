//! # RnR
//!
//! `rnr` is a command-line tool to batch rename files for ANSI terminals.
//!
extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate walkdir;

use renamer::Renamer;

mod app;
mod fileutils;
mod output;
mod renamer;
mod solver;

fn main() {
    // Read arguments
    let config = match app::Config::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    if !config.force {
        let info = &config.printer.colors.info;
        config
            .printer
            .print(&format!("{}", info.paint("This is a DRY-RUN")));
    }

    // Configure renamer
    let mut renamer = Renamer::new(&config);

    // Process files
    renamer.process();
}
