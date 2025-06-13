use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    // Only compile assembly files for x86_64 targets
    if target_arch == "x86_64" && cfg!(feature = "gnark-optimized") {
        // Assembly files are only compatible with Linux and potentially other Unix systems
        // They use AT&T syntax which may not work on all platforms
        if target_os == "linux" || target_os == "freebsd" || target_os == "netbsd" {
            println!("cargo:rerun-if-changed=src/bn128/assembly/bn254_add_sub_amd64.s");
            println!("cargo:rerun-if-changed=src/bn128/assembly/bn254_mul_amd64.s");
            
            // Use cc crate to compile assembly files
            cc::Build::new()
                .file("src/bn128/assembly/bn254_add_sub_amd64.s")
                .file("src/bn128/assembly/bn254_mul_amd64.s")
                .flag("-x")
                .flag("assembler")
                .compile("bn254_asm");
                
            println!("cargo:warning=BN254 assembly optimizations enabled for {}", target_os);
        } else {
            // On macOS and Windows, we can't use the AT&T syntax assembly files
            // Instead, we'll rely on the Rust fallback implementations
            println!("cargo:rustc-cfg=no_asm");
            println!("cargo:warning=BN254 assembly optimizations disabled on {}. Only Linux/BSD supports the assembly files.", target_os);
        }
    }
}