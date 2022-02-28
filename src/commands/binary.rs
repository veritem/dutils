use clap::ArgMatches;

pub fn convert_to_binary(matches: &ArgMatches) {
    if let Some(number) = matches.value_of("decimal") {
        println!("{}", number);
    }
}
