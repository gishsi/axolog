use clap::Parser;
use crate::commonlogtypes::CommonLogTypes;

///  Utility tool for parsing Minecraft server and client logs into structured format.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the log file
    #[arg(long)]
    pub path_to_file: String,

    /// Path to save the file with the structured data at. If empty, the formatted output in JSON form gets written to the console.
    #[arg(long, default_value="")]
    pub path_to_save: String,

    #[arg(long)]
    #[clap(value_enum, default_value_t=CommonLogTypes::All)]
    pub log_type: CommonLogTypes,
}