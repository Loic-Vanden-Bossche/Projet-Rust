use clap::Parser;
use log::{info, LevelFilter};
use crate::types::args::Args;

pub fn parse_args() -> Option<(Option<String>, u32, LevelFilter, Option<String>)>{
    let args = Args::parse();
    if args.no_ui {
        info!("ICII");
        Some((args.name, args.port, if args.debug { LevelFilter::Debug } else { LevelFilter::Info }, args.host))
    }else{
        None
    }
}