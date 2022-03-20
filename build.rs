/*
    ISPC project file builds the kernels as such:
    <Command Condition="'$(Configuration)|$(Platform)'=='Release|x64'">ispc -O2 "%(Filename).ispc" -o "$(TargetDir)%(Filename).obj" -h "$(ProjectDir)%(Filename)_ispc.h" --target=sse2,sse4,avx,avx2 --opt=fast-math</Command>
    <Outputs Condition="'$(Configuration)|$(Platform)'=='Release|x64'">$(TargetDir)%(Filename).obj;$(TargetDir)%(Filename)_sse2.obj;$(TargetDir)%(Filename)_sse4.obj;$(TargetDir)%(Filename)_avx.obj;$(TargetDir)%(Filename)_avx2.obj;</Outputs>
*/

#[cfg(feature = "ispc")]
fn compile_kernel() {
    use std::env;

    use ispc_compile::TargetISA;

    println!("cargo:rerun-if-changed=src/ispc/*");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
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
        .file("thirdparty/ISPCTextureCompressor/ispc_texcomp/kernel.ispc")
        .opt_level(2)
        .optimization_opt(ispc_compile::OptimizationOpt::FastMath)
        .target_isas(target_isas.clone())
        .out_dir("src/ispc")
        .compile("kernel");

    ispc_compile::Config::new()
        .file("thirdparty/ISPCTextureCompressor/ispc_texcomp/kernel_astc.ispc")
        .opt_level(2)
        .optimization_opt(ispc_compile::OptimizationOpt::FastMath)
        .target_isas(target_isas)
        .out_dir("src/ispc")
        .compile("kernel_astc");
}

#[cfg(not(feature = "ispc"))]
fn compile_kernel() {
    ispc_rt::PackagedModule::new("kernel")
        .lib_path("src/ispc")
        .link();

    ispc_rt::PackagedModule::new("kernel_astc")
        .lib_path("src/ispc")
        .link();
}

fn main() {
    compile_kernel();
}
