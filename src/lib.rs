mod config;
use color_print::cformat;
use std::{fmt::Display, process};

pub fn raise<E: Display>(msg: E) -> ! {
    eprintln!("{}: {msg}", cformat!("<r><s>error</>"));
    process::exit(1);
}
