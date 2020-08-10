extern crate cc;

use std::process::Command;

fn main() {
    if cfg!(tarfet_os = "linux") {
        Command::new("make")
            .current_dir("lua")
            .status()
            .expect("Lua Make failed");

        cc::Build::new()
            .flag("-I")
            .flag("lua")
            .flag("-llua")
            .file("c_drivers/lua_vm.c")
            .compile("lua_vm");

        println!("cargo:rustc-flags=-l lua -L lua");
    }
}
