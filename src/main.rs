use std::fs::OpenOptions;
use std::io::prelude::*;
mod macros;
fn main() {
    Vita_App!(
        "./styles.css",
        body{
            color:"blue"
        }
    );
}
