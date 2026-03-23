use std::fs::OpenOptions;

use dbgout::{Color, debug, set_debug_color, set_debug_writer};

fn do_debug_prints(prestr: &str) {
    // normal debug print
    debug!("{} - this is a normal debug print", prestr);
    // debug print which also activates on debug build
    debug!(
        auto,
        "{} - this will print on both a debug build, and the --debug flag", prestr
    );
    // this debug print will always print
    debug!(true, "{} - this will always print", prestr);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // just a regular debug print
    do_debug_prints("regular");

    eprintln!();

    // set the colour
    set_debug_color(Color::Blue);
    do_debug_prints("blue");

    // send to a file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")?;
    set_debug_writer(Box::new(file));
    do_debug_prints("file");

    Ok(())
}
