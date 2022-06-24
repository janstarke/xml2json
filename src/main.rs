
use clap::Parser;
use anyhow::Result;
use file_type::FileType;
use quick_xml::Reader;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};
use std::io::{BufReader};

mod xmltojson;
mod file_type;

#[derive(Parser, Clone)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// path of the XML file to read from; we use '-' to denote STDIN
    #[clap(short('X'), long("xml-file"), default_value_t = FileType::Stream)]
    xml_file: FileType,

    /// path of the JSON file to write to; we use '-' to denote STDIN
    #[clap(short('J'), long("json-file"), default_value_t = FileType::Stream)]
    json_file: FileType
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto)?;
        

    let input_stream = cli.xml_file.as_reader()?;
    let output_stream = cli.json_file.as_writer()?;

    let mut reader = Reader::from_reader(BufReader::new(input_stream));
    let val = xmltojson::read(&mut reader, 0);

    serde_json::ser::to_writer(output_stream, &val)?;
    Ok(())
}
