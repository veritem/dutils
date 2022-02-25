use clap::Command;

mod commands;

fn main() {
    let matches = Command::new("dutils")
        .version("0.1.0")
        .author("Makuza Mugabo Verite")
        .subcommand(Command::new("c").about("Convert between binary and decimal"))
        .subcommand(Command::new("s").about("Econder for base64"))
        .subcommand(Command::new("z").about("Convert time zones"))
        .subcommand(Command::new("c").about("Convert between time formats"))
        .subcommand(Command::new("c").about("Hash conversions"))
        .subcommand(Command::new("g").about("Utils to handle passwords"))
        .get_matches();

    match matches.subcommand_name() {
        Some("bs") => {
            // commands::bs;
        }
        _ => unreachable!(),
    }
}
