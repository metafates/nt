mod storage;

extern crate copypasta;

use clap::{command};
use copypasta::{ClipboardContext, ClipboardProvider};


fn main() {
    let mut ctx = ClipboardContext::new().unwrap();


    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            command!("set")
                .about("set a new record")
                .arg(
                    clap::Arg::with_name("name")
                        .help("name of the record")
                        .required(true)
                        .index(1),
                )
                .arg(
                    clap::Arg::with_name("value")
                        .help("value of the record")
                        .required(false)
                        .index(2),
                ),
        )
        .subcommand(
            command!("get")
                .about("get record")
                .arg(
                    clap::Arg::with_name("name")
                        .help("name of the record")
                        .required(true)
                        .index(1),

                )
                .arg(
                    clap::Arg::with_name("copy")
                        .help("copy value to the clipboard")
                        .short('c')
                        .long("copy")
                        .takes_value(false),
                )
        )
        .subcommand(
            command!("remove")
                .about("remove a record")
                .alias("rm")
                .arg(
                    clap::Arg::with_name("name")
                        .help("name of the record")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            command!("list")
                .about("list all records")
                .alias("ls")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", matches)) => {
            let name = match matches.get_one::<String>("name") {
                Some(value) => value,
                None => unreachable!(),
            };

            match matches.get_one::<String>("value") {
                Some(value) => {
                    storage::set_record(name, value).unwrap();
                    println!("'{}' set to '{}'", name, value);
                },
                None => match ctx.get_contents() {
                    Ok(value) => {
                        storage::set_record(name, &value).unwrap();
                        println!("'{}' set to contents from clipboard", name);
                    },
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                },
            };

        },
        Some(("get", matches)) => {
            let name = match matches.get_one::<String>("name") {
                Some(value) => value,
                None => unreachable!(),
            };

            match storage::get_record(&name).unwrap() {
                (_, Some(value)) => {
                    if matches.is_present("copy") {
                        ctx.set_contents(value).unwrap();
                        println!("copied to clipboard");
                    } else {
                        println!("{}", value);
                    }
                },
                (name, None) => println!("'{}' not found", name),
            }

        },
        Some(("remove", matches)) => {
            let name = match matches.get_one::<String>("name") {
                Some(value) => value,
                None => unreachable!(),
            };

            storage::remove_record(name).unwrap();
            println!("'{}' removed", name);
        },
        Some(("list", _)) => {
            let records = storage::get_records().unwrap();
            for record in records {
                println!("'{}' - '{}'\n", record.0, record.1);
            }
        },
        _ => unreachable!(),
    };

}
