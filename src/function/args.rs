use clap::Parser;
use log::LevelFilter;
use crate::types::args::Args;

pub fn parse_args() -> (String, u32, LevelFilter, Option<String>){
    let args = Args::parse();
    (args.name, args.port, if args.debug { LevelFilter::Debug } else { LevelFilter::Info }, args.host)
}