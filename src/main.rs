use clap::Command;

mod commands;

fn main() {
    let matches = Command::new("dutils")
        .author("\nAuthor: Makuza Mugabo Verite")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("binary")
                .short_flag('b')
                .about("converters")
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("base64")
            .about("enconder and decoder for base64")
            .short_flag('a')
            .arg_required_else_help(true))
        .subcommand(
            Command::new("hash")
            .about("Hash conversions")
            .short_flag('h')
            .arg_required_else_help(true)
        ) 
        .subcommand(
            Command::new("timezone")
                .about("Handle timezones")
                .short_flag('t')
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("generate")
                .short_flag('g')
                .about("generation commands")
                // .subcommand("password")
                // .short_flag('g')
                // .about("generate password"),
        )
        .subcommand(
            Command::new("image")
            .short_flag('i')
            .about("Convert images")
            )
        .get_matches();

    match matches.subcommand_name() {
        Some("c") => {
            // commands::bs;
        }
        _ => unreachable!(),
    }
}
