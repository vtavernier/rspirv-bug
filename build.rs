use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let source = fs::read_to_string("glsl/opt.comp").unwrap();
    let mut compiler = shaderc::Compiler::new().unwrap();

    let dest = PathBuf::from(env::var("OUT_DIR").unwrap());
    let binary_result = compiler
        .compile_into_spirv(
            &source,
            shaderc::ShaderKind::Compute,
            "glsl/opt.comp",
            "main",
            None,
        )
        .unwrap();

    fs::write(dest.join("opt.spv"), binary_result.as_binary_u8()).unwrap();
}
