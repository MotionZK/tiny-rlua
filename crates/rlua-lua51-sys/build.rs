extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let lua_folder = "lua-5.1.5";

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let libc_dir: PathBuf = match env::var_os("LIBC_DIR") {
        Some(libc) => libc.into(),
        None => {
            let mut lib_path = PathBuf::from(env::var("HOME").unwrap());
            lib_path.push("riscv32im-linux-x86_64/"); // https://github.com/risc0/toolchain/releases/tag/2022.03.25
            lib_path
        }
    };

    let libc = libc_dir.join("picolibc/riscv32-unknown-elf/lib/");
    let include = libc_dir.join("picolibc/riscv32-unknown-elf/include/");
    let sys_include = libc_dir.join("picolibc/riscv32-unknown-elf/sys-include/");
    let toolchain = libc_dir.join("riscv32-unknown-elf/");

    let lua_dir = PathBuf::from(lua_folder).join("src");
    let mut cc_config = cc::Build::new();

    let cc_config = cc_config
        .define("LUA_USE_C89", None)
        .warnings(false);

    let cc_config_build = cc_config
        .compiler("clang")
        .include(&include)
        .include(&sys_include)
        .include(&lua_dir)
        .include(&toolchain)
        .target("riscv32-unknown-elf")
        .flag(&format!("--gcc-toolchain={}", libc_dir.to_str().unwrap()) as &str);

    let target_features = "-a -c +m".split(" ");
    for flag in target_features {
        let tokens = format!("-Xclang -target-feature -Xclang {}", flag);
        for token in tokens.split(" ") {
            cc_config_build.flag(token);
        }
    }

    println!("cargo:rustc-link-search={}", libc.display());
    println!("cargo:rustc-link-lib=static={}", "c");
    println!("cargo:rerun-if-env-changed=LIBC_DIR");
    println!("cargo:rerun-if-changed={}", lua_dir.display());
    
    cc_config_build
        .file(lua_dir.join("lapi.c"))
        .file(lua_dir.join("lbaselib.c"))
        .file(lua_dir.join("lcode.c"))
        .file(lua_dir.join("ldblib.c"))
        .file(lua_dir.join("ldebug.c"))
        .file(lua_dir.join("ldo.c"))
        .file(lua_dir.join("ldump.c"))
        .file(lua_dir.join("lfunc.c"))
        .file(lua_dir.join("lgc.c"))
        .file(lua_dir.join("liolib.c"))
        .file(lua_dir.join("llex.c"))
        .file(lua_dir.join("lmathlib.c"))
        .file(lua_dir.join("lauxlib.c"))
        .file(lua_dir.join("lmem.c"))
        .file(lua_dir.join("loadlib.c"))
        .file(lua_dir.join("lobject.c"))
        .file(lua_dir.join("lopcodes.c"))
        .file(lua_dir.join("lparser.c"))
        .file(lua_dir.join("lstate.c"))
        .file(lua_dir.join("lstring.c"))
        .file(lua_dir.join("lstrlib.c"))
        .file(lua_dir.join("ltable.c"))
        .file(lua_dir.join("ltablib.c"))
        .file(lua_dir.join("ltm.c"))
        .file(lua_dir.join("lundump.c"))
        .file(lua_dir.join("lvm.c"))
        .file(lua_dir.join("lzio.c"));

    cc_config_build
        .out_dir(dst.join("lib"))
        .compile("lua5.1"); 
}
