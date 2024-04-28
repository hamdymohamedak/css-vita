#[macro_export]
macro_rules! Vita_App {
    ($file:expr, $($code:tt)*) => {
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        let file_path = $file;

        if file_path.ends_with(".css") {
            let mut code = String::new();
            let comment = "/* Created By css-Vita */";





        // this is the code will show at css
        //LOGIC HERE

        code.push_str(&format!("{}", stringify! {$($code)*}));

        //LOGIC HERE
        // this is the code will show at css














            if let Ok(mut file) = OpenOptions::new().append(true).open(&file_path) {
                if let Err(e) = writeln!(file, "{} {}",comment,code) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("Code appended to the CSS file successfully!");
                }
            } else {
                eprintln!("Error opening the file");
            }
        } else {
            eprintln!("The provided file is not a CSS file.");
        }
    };
}
