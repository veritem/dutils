use clap::{App, AppSettings};

mod commands;

fn main() {
    let matches = App::new("dutils")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Makuza Mugabo Verite")
        .subcommand(App::new("bc").about("Convert between binary and decimal"))
        .subcommand(App::new("bs").about("Econder for base64"))
        .subcommand(App::new("tz").about("Convert time zones"))
        .subcommand(App::new("tc").about("Convert between time formats"))
        .subcommand(App::new("hc").about("Hash conversions"))
        .subcommand(App::new("pg").about("Utils to handle passwords"))
        .get_matches();

    match matches.subcommand_name() {
        Some("bs") => {
            // commands::bs;
        }
        _ => unreachable!(),
    }
}
