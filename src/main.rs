use clap::{App, AppSettings, Arg, SubCommand, ArgMatches};
use unic_ucd::*;
use unicode_cli::*;
use regex::RegexBuilder;
use log::*;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::InferSubcommands)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Output more information."),
        )
        .subcommand(
            SubCommand::with_name("compose")
                .alias("c")
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
                )
                .arg(
                    Arg::with_name("long")
                        .short("l")
                        .long("long")
                        .help("Output more detail."),
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
        .subcommand(
            SubCommand::with_name("search")
                .about("Search for unicode characters by name or properties.")
                .arg(
                    Arg::with_name("REGEX")
                        .required(true)
                        .help("Unicode character."),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("List unicode characters")
                .arg(
                    Arg::with_name("long")
                        .short("l")
                        .long("long")
                        .help("Print details for each character."),
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Print all characters."),
                )
                .arg(
                    Arg::with_name("BLOCK")
                        .help("Which block to display."),
                ),
        )
        .get_matches();

    stderrlog::new()
	.module(module_path!())
	.quiet(matches.is_present("quiet"))
	.timestamp(stderrlog::Timestamp::Second)
	.verbosity(matches.occurrences_of("verbose") as usize)
	.init()
	.unwrap();

    match matches.subcommand() {
        ("compose", Some(args)) => compose(args),
        ("inspect", Some(args)) => inspect(args),
        ("info", Some(args)) => info(args),
        ("list", Some(args)) => list(args),
        ("search", Some(args)) => search(args),
        _ => unreachable!(),
    }
}

fn compose(args: &ArgMatches) {
    let mut composed = String::new();

    if let Some(values) = args.values_of("NAME") {
        for name in values {
            if let Some(point) = parse_scalar_value(name) {
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
    let charinfo = CharInfo::new(args.occurrences_of("long") as usize + 2);

    for chr in composed.chars() {
        charinfo.display(chr);
    }
}

fn info(args: &ArgMatches) {
    let composed = args.value_of("CHARACTER").unwrap();

    if let Some(c) = parse_scalar_value(composed) {
        if let Some(info) = Info::of(c) {
            println!("{}", &info);
        } else {
            println!("error: '{}' ({}) is not recognized.", c, c as u32);
        }
    } else {
        println!("error: can't parse {}.", composed);
    }
}

fn list(args: &ArgMatches) {
    if let Some(block) = args.value_of("BLOCK") {
        if let Some(block) = BlockIter::default().find(|b| b.name == block) {
            println!("{}", block.name);

            for c in block.range {
                print!("{}", c);
            }

            println!("");
        } else {
            println!("error: can't find block {}", block);
        }
    } else {
        for block in BlockIter::default() {
            println!("{}", block.name);
        }
    }
}

fn search(args: &ArgMatches) {
    let regex = args.value_of("REGEX").unwrap();
    let regex = RegexBuilder::new(regex)
        .case_insensitive(true)
        .build()
        .unwrap();

    let res = unicode_cli::search(&regex);

    for c in res.iter() {
        println!("{}", c);
    }
}
