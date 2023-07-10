// remove all unused warnings
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
///////////
use std::env;
use clap::{arg, command, Command, ArgAction, Arg};
///////////
mod setup;
mod deserializer;
mod general;
mod template;
mod figma_parser;
mod utils;
mod global;
mod filters {
    pub mod as_text_or_number;
    pub mod remove_space;
    pub mod color;
    pub mod case;
    pub mod optional;
    pub mod empty;
}
////////////
/// 
// cargo run --release  --  --generate --config "assets/design_tokens_config.yaml"
fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    // Covert to usable code
    let matches: clap::ArgMatches = Command::new("Design Tokens")
        .version("3.1.5")
        .author("Vladislav R. <vrrashkov@gmail.com>")
        .about("Parses figma design tokens to usable code")
        .arg(Arg::new("config").short('c').long("config").action(ArgAction::Set).required(true))
        .arg(Arg::new("generate").short('g').long("generate").action(ArgAction::SetTrue))
        .get_matches();

    let config_file = matches.get_one::<String>("config").expect("required");

    // println!(
    //     "config: {:?}",
    //     &config_file
    // );

    // Deserialize the config file
    let token_config: deserializer::TokensConfig = general::get_config(config_file);

    if let Some(value) = matches.get_one::<bool>("generate") {
        if *value {
            // Generate styles
            figma_parser::filter_properties(&token_config);
        }
    }
    setup::init(&token_config);

}
