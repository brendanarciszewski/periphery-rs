use bindgen;
use cc::Build;
use make_cmd::make;
use std::env;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut command = make();
    command.arg(format!("OBJDIR={}", out_dir.to_str().unwrap()));
    command.arg(format!(
        "LIB={}",
        out_dir.join("libperiphery.a").to_str().unwrap()
    ));
    let periphery_dir = env::current_dir().unwrap().join("src/c-periphery");
    command.args(&["-C", periphery_dir.to_str().unwrap()]);
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
    match command
        .current_dir(periphery_dir.clone())
        .status()
        .unwrap()
        .code()
    {
        None => exit(-1),
        Some(0) => (),
        Some(code) => exit(code),
    };

    let periphery_src_dir = periphery_dir.join("src");
    let bindings = bindgen::Builder::default()
        .header(periphery_src_dir.join("gpio.h").to_str().unwrap())
        .header(periphery_src_dir.join("i2c.h").to_str().unwrap())
        .header(periphery_src_dir.join("mmio.h").to_str().unwrap())
        .header(periphery_src_dir.join("serial.h").to_str().unwrap())
        .header(periphery_src_dir.join("spi.h").to_str().unwrap())
        .header(periphery_src_dir.join("version.h").to_str().unwrap())
        .generate()
        .unwrap();
    bindings.write_to_file(out_dir.join("bindings.rs")).unwrap();
    println!("cargo:rustc-link-lib=static=periphery");
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );
}
