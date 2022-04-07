/*
    ISPC project file builds the kernels as such:
    <Command Condition="'$(Configuration)|$(Platform)'=='Release|x64'">ispc -O2 "%(Filename).ispc" -o "$(TargetDir)%(Filename).obj" -h "$(ProjectDir)%(Filename)_ispc.h" --target=sse2,sse4,avx,avx2 --opt=fast-math</Command>
    <Outputs Condition="'$(Configuration)|$(Platform)'=='Release|x64'">$(TargetDir)%(Filename).obj;$(TargetDir)%(Filename)_sse2.obj;$(TargetDir)%(Filename)_sse4.obj;$(TargetDir)%(Filename)_avx.obj;$(TargetDir)%(Filename)_avx2.obj;</Outputs>
*/

#[cfg(feature = "ispc")]
use glob::glob;
#[cfg(feature = "ispc")]
use std::fs;

#[cfg(feature = "ispc")]
fn clean_target(target_name: &str, target: &str) {
    let target_files = format!("src/ispc/*{}{}*", target_name, target);
    println!("remove lib kernel {}", target_files);
    for entry in glob(target_files.as_str()).unwrap() {
        match entry {
            Ok(target_file) => println!("{:?}", fs::remove_file(target_file).unwrap()),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[cfg(feature = "ispc")]
fn compile_kernel() {
    use ispc_compile::TargetISA;
    use std::env;

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target = env::var("TARGET").unwrap();
    println!("cargo:rerun-if-changed=src/kernels/*");

    clean_target("kernel", &target);
    clean_target("kernel_astc", &target);

    let target_isas = match target_arch.as_str() {
        "x86" | "x86_64" => vec![
            TargetISA::SSE2i32x4,
            TargetISA::SSE4i32x4,
            TargetISA::AVX1i32x8,
            TargetISA::AVX2i32x8,
            TargetISA::AVX512KNLi32x16,
            TargetISA::AVX512SKXi32x16,
        ],
        "aarch64" | "arm" => vec![TargetISA::Neoni32x4],
        _ => vec![],
    };

    ispc_compile::Config::new()
        .file("src/kernels/kernel.ispc")
        .optimization_opt(ispc_compile::OptimizationOpt::FastMath)
        .target_isas(target_isas.clone())
        .out_dir("src/ispc")
        .compile("kernel");

    ispc_compile::Config::new()
        .file("src/kernels/kernel_astc.ispc")
        .optimization_opt(ispc_compile::OptimizationOpt::FastMath)
        .target_isas(target_isas)
        .out_dir("src/ispc")
        .compile("kernel_astc");

    let target_os_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();

    if target_os_family == "windows" {
        let outdir = env::var("OUT_DIR").unwrap();
        cc::Build::new()
            .cpp(true)
            .file("src/ispc/ispc_texcomp_astc.cpp")
            .include(outdir)
            .out_dir("src/ispc")
            .compile(&format!("ispc_texcomp_astc{}", target));
    }
}

#[cfg(not(feature = "ispc"))]
fn compile_kernel() {
    use std::env;
    ispc_rt::PackagedModule::new("kernel")
        .lib_path("src/ispc")
        .link();

    ispc_rt::PackagedModule::new("kernel_astc")
        .lib_path("src/ispc")
        .link();

    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        println!("cargo:rustc-link-search=native=src/ispc");
        println!(
            "cargo:rustc-link-lib=static=ispc_texcomp_astc{}",
            env::var("TARGET").unwrap()
        );
    }
}

fn main() {
    compile_kernel();
}
