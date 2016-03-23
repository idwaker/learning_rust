
extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let matches = parse_args();

    // shadowing what is ?
    if let Some(ref matches) = matches.subcommand_matches("month") {
        let day_of_week = match matches.value_of("day") {
            Some(day_of_week) => day_of_week,
            None => {
                println!("month requires day and ordinal");
                return;
            }
        };

        let ordinal = match matches.value_of("ord") {
            Some(ordinal) => ordinal,
            None => {
                println!("month requires day and ordinal");
                return;
            },
        };

        println!("{} {}", day_of_week, ordinal);
    }
}

// what are lifetime parameters ?
fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("reldate")
        .version("0.1.0")
        .author("Diwaker Ghimire <idwaker@gmail.com>")
        .about("Print relative dates")
        .subcommand(SubCommand::with_name("month")
                .about("Allows creation of month relative date streams")
                .arg(Arg::with_name("day")
                        .short("d")
                        .help("day of week")
                        .takes_value(true))
                .arg(Arg::with_name("ord")
                        .short("o")
                        .long("ord")
                        .help("ordinal form of repeated value")
                        .takes_value(true)))
        .get_matches()

        // .arg(Arg::new("month")
        //         .short("m")
        //         .long("month")
        //         .help("specifies month relative date")
        //         .takes_value(false))
        // .arg(Arg::new("week")
        //         .short("w")
        //         .long("week")
        //         .help("specifies week relative date")
        //         .takes_value(false))
        // .arg(Arg::new("year")
        //         .short("y")
        //         .long("year")
        //         .help("specifies year relative date")
        //         .takes_value(false));
}