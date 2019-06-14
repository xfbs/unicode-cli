use unic_ucd::Block;
use clap::{App, AppSettings, Arg, SubCommand, ArgMatches};
use unicode_cli::*;

fn main() {
    let matches = App::new("unicode-cli")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Patrick Elsen <pelsen@xfbs.net>")
        .about("Unicode character lookup and inspection tool.")
        .subcommand(
            SubCommand::with_name("compose")
                .about("Compose a string from unicode codepoins.")
                .arg(
                    Arg::with_name("NAME")
                        .required(true)
                        .multiple(true)
                        .help("Name of a unicode codepoint"),
                ),
        )
        .subcommand(
            SubCommand::with_name("inspect")
                .about("Inspect a unicode string.")
                .arg(
                    Arg::with_name("STRING")
                        .required(true)
                        .help("String containing unicode data."),
                ),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Print information about a unicode character.")
                .arg(
                    Arg::with_name("CHARACTER")
                        .required(true)
                        .help("Unicode character."),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("compose", Some(args)) => compose(args),
        ("inspect", Some(args)) => inspect(args),
        ("info", Some(args)) => info(args),
        _ => unreachable!(),
    }
}

fn compose(args: &ArgMatches) {
    let mut composed = String::new();

    if let Some(values) = args.values_of("NAME") {
        for name in values {
            println!("inspecting {}", name);
            if let Some(point) = unicode_names2::character(name) {
                composed.push(point);
            } else {
                println!("{} is not a name of a unicode code point.", name);
            }
        }
    } else {
        println!("can't extract values of NAME");
    }

    println!("{}", composed);
}

fn inspect(args: &ArgMatches) {
    let composed = args.value_of("STRING").unwrap();

    for chr in composed.chars() {
        if let Some(name) = unicode_names2::name(chr) {
            println!("{:?}", name);
        }

        if let Some(block) = Block::of(chr) {
            println!("block: {}", block.name);
        }
    }
}

fn info(args: &ArgMatches) {
    let composed = args.value_of("CHARACTER").unwrap();

    if let Some(c) = parse_scalar_value(composed) {
        if let Some(info) = Info::of(c) {
            println!("{}", &info);
        } else {
            println!("error: {} is not recognized.", c);
        }
    } else {
        println!("error: can't parse {}.", composed);
    }
}
