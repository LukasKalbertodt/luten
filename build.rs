use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{self, ErrorKind, Write};

/// The folder containing `main.scss`.
const SCSS_DIR: &str = "scss/";

/// The output folder in which `main.css` will be written.
const CSS_OUT_DIR: &str = "static/";


fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    compile_sass(&manifest_dir);
}

/// Compiles `.scss` files with `sass` which is assumed to be installed.
fn compile_sass(manifest_dir: &Path) {
    // In and out paths
    let scss_dir = manifest_dir.join(SCSS_DIR);
    let out_dir = manifest_dir.join(CSS_OUT_DIR);

    // If the application is being compiled in release mode, we want to minify
    // the resulting CSS.
    //
    // We might want to check for Rocket environments instead of the cargo
    // build profile later (TODO, see Rocket#228).
    let is_debug = env::var("PROFILE").unwrap() == "debug";
    let minify_flags = ["-t", "compressed"];
    let flags = if is_debug { &[] as &[_] } else { &minify_flags };

    // Execute the compiler
    let res = Command::new("sass")
        .args(flags)
        .arg(&scss_dir.join("main.scss"))
        .arg(&out_dir.join("main.css"))
        .output();

    // Check if anything went wrong
    match res {
        Err(e) => {
            eprintln!("An IO error occured while running the scss-compiler:");
            eprintln!(" >> {}", e);

            if e.kind() == ErrorKind::NotFound {
                eprintln!("!! Make sure you have installed `sass` and it's in your $PATH! !!");
            }
            eprintln!("");

            panic!("error compiling scss files");
        }
        Ok(output) => {
            // If everything went well, we don't expect any output
            if !output.status.success() || !output.stdout.is_empty() || !output.stderr.is_empty() {
                eprintln!("`sass` exited unsuccessful!");

                eprintln!("--- stdout ---");
                io::stderr().write_all(&output.stdout)
                    .expect("IO error printing to stderr");
                eprintln!("--- stderr ---");
                io::stderr().write_all(&output.stderr)
                    .expect("IO error printing to stderr");

                panic!("error compiling scss files");
            }
        }
    }
}
