fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let plist = format!("{manifest_dir}/Info.plist");
    println!("cargo:rerun-if-changed=Info.plist");
    // 把 Info.plist 嵌进 Mach-O 的 __TEXT,__info_plist 段
    println!("cargo:rustc-link-arg=-Wl,-sectcreate,__TEXT,__info_plist,{plist}");
}
