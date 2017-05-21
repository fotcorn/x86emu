extern crate clap;
use clap::{App, Arg};

extern crate x86emu;
use x86emu::loader::elf::elf;
use x86emu::loader::linux::linux;

fn main() {
    let matches = App::new("x86emu")
        .arg(Arg::with_name("file").required(true))
        .arg(Arg::with_name("symbol")
            .help("symbol to execute in elf file")
            .long("symbol")
            .short("s")
            .takes_value(true))
        .arg(Arg::with_name("loader")
            .help("binary loader type")
            .long("loader")
            .short("l")
            .takes_value(true)
            .possible_values(&["linux", "elf"]))
        .arg(Arg::with_name("debug")
            .help("run in debug mode (print all registers after every instruction)")
            .long("debug")
            .short("d"))
        .arg(Arg::with_name("benchmark")
            .help("print how long it took to execute the main loop")
            .long("benchmark")
            .short("b"))
        .arg(Arg::with_name("print-instructions")
            .help("print every executed instruction")
            .long("print-instructions")
            .short("p"))
        .get_matches();

    let symbol = matches.value_of("symbol").unwrap_or("main");
    let loader = matches.value_of("loader").unwrap_or("elf");
    let filename = matches.value_of("file").unwrap();
    let debug = matches.is_present("debug");
    let benchmark = matches.is_present("benchmark");
    let print_instructions = matches.is_present("print-instructions");

    match loader {
        "linux" => {
            linux(filename, debug);
        }
        "elf" => {
            elf(filename, symbol, debug, print_instructions, benchmark);
        }
        _ => unreachable!("Values already validated by clap"),
    }

}
