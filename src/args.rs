use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[arg(value_parser = clap::value_parser!(u32).range(3..))]
    pub boxes: u32,

    pub morty_name: String,
}
