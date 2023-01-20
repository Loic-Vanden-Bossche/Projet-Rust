use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // The port to connect to
    #[arg(short, long, default_value_t = 7878)]
    pub port: u32,

    // Mandatory if --no-ui is present
    pub name: Option<String>,

    // The host to connect to
    pub host: Option<String>,

    // Print debug logs (only with --no-ui)
    #[arg(long)]
    pub debug: bool,

    // Launch without ui
    #[arg(long)]
    pub no_ui: bool
}