use clap::Parser;
use log::{LevelFilter};
use crate::types::args::Args;

pub fn parse_args() -> (Option<String>, u32, LevelFilter, Option<String>, bool){
    let args = Args::parse();
    (args.name, args.port, if args.debug { LevelFilter::Debug } else { LevelFilter::Info }, args.host, args.no_ui)
}