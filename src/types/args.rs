use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 7878)]
    pub port: u32,

    pub name: String,

    pub host: Option<String>,

    #[arg(long)]
    pub debug: bool
}