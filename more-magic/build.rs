fn main() {
    cc::Build::new()
        .file("src/file.c")
        .compile("moremagic");

    println!("cargo:rerun-if-changed=src/file.c");
    println!("cargo:rustc-link-lib=dylib=magic");
    println!("cargo:rustc-link-lib=dylib=udev");
}
