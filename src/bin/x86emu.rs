extern crate clap;
use clap::{App, Arg};

extern crate x86emu;
use x86emu::cpu::print::PrintCPU;
use x86emu::cpu::emu_instructions::EmulationCPU;
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
        .arg(Arg::with_name("cpu")
            .help("cpu to execute")
            .long("cpu")
            .short("c")
            .takes_value(true)
            .possible_values(&["emu", "print"]))
        .arg(Arg::with_name("debug")
            .help("run in debug mode (single step, print all registers after every instruction)")
            .long("debug")
            .short("d"))
        .get_matches();

    let symbol = matches.value_of("symbol").unwrap_or("main");
    let cpu = matches.value_of("cpu").unwrap_or("print");
    let loader = matches.value_of("loader").unwrap_or("elf");
    let filename = matches.value_of("file").unwrap();
    let debug = matches.is_present("debug");

    match loader {
        "linux" => {
            match cpu {
                "print" => linux(filename, &PrintCPU {}, debug),
                "emu" => linux(filename, &EmulationCPU {}, debug),
                _ => unreachable!("Values already validated by clap"),
            };
        }
        "elf" => {
            match cpu {
                "print" => elf(filename, symbol, &PrintCPU {}, debug),
                "emu" => elf(filename, symbol, &EmulationCPU {}, debug),
                _ => unreachable!("Values already validated by clap"),
            };
        }
        _ => unreachable!("Values already validated by clap"),
    }

}
