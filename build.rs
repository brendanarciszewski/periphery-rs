use cc::Build;
use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut command = Command::new("make");
    command.arg(format!("OBJDIR={}", out_dir));
    command.arg(format!(
        "LIB={}",
        [out_dir.as_str(), "libperiphery.a"]
            .iter()
            .collect::<PathBuf>()
            .to_str()
            .unwrap()
    ));
    let mut src_dir = env::current_dir().unwrap();
    src_dir.push("src/c-periphery");
    let src_dir = src_dir.to_str().unwrap();
    command.args(&["-C", src_dir]);
    let build = Build::new();
    let compiler = build.get_compiler();
    match &compiler.cc_env() {
        cc if cc != "" => {
            command.arg(format!("CC={}", cc.to_str().unwrap()));
        }
        _ => (),
    };
    match &compiler.cflags_env() {
        cflags if cflags != "" => {
            command.arg(format!("CFLAGS={}", cflags.to_str().unwrap()));
        }
        _ => (),
    };
    match command.current_dir(src_dir).status().unwrap().code() {
        None => exit(-1),
        Some(code) => exit(code),
    }
}
