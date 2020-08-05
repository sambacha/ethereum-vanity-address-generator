extern crate docopt;
extern crate simplelog;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

mod lib;

use crate::lib::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
    get_tool_version_info::get_tool_version_info,
    generate_random_address::generate_random_address,
    generate_vanity_address::generate_vanity_address,
    initialize_logger::maybe_initialize_logger_and_return_cli_args,
    get_cli_args::{
        CliArgs,
        get_cli_args,
    },
};

/// # For usage info, please see the __`README.md`__ of the repo
pub fn main() -> Result<()> {
    match get_cli_args()
        .and_then(maybe_initialize_logger_and_return_cli_args)
        .and_then(|cli_args| -> Result<String> {
            match cli_args {
                CliArgs {cmd_generateAddress: true, ..} => {
                    info!("✔ Generating random address...");
                    generate_random_address()
                }
                CliArgs {cmd_generateVanityAddress: true, ..} => {
                    info!("✔ Generating vanity address...");
                    generate_vanity_address(&cli_args.arg_prefix)
                }
                CliArgs {cmd_version: true, ..} => {
                    info!("✔ Getting tool version info...");
                    get_tool_version_info()
                }
                _ => Err(AppError::Custom(USAGE_INFO.to_string()))
            }
        }) {

            Ok(result) => {
                println!("{:?}", result);
                Ok(())
            },
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        }
}