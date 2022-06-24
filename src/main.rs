use clap::Parser;
use anyhow::Result;

#[derive(Parser, Clone)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// path of the XML file to read from; '-' denotes STDIN, which is the default
    #[clap(short('X'), long("xml-file"))]
    xml_file: Option<String>,

    /// path of the JSON file to write to; '-' denotes STDOUT, which is the default
    #[clap(short('J'), long("json-file"))]
    json_file: Option<String>
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    Ok(())
}
