use std::fs::OpenOptions;
use std::io::prelude::*;
mod macros;
fn main() {
    Vita_App!(
        "/media/ak/96BC3336BC330FEB/cargo_projects/css-vita/css-vita/style.css",
        body{
            color:"blue"
        }
    );
}
