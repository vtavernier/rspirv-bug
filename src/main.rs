extern crate rspirv;
extern crate spirv_headers as spirv;

fn main() {
    let mut loader = rspirv::dr::Loader::new();
    rspirv::binary::parse_bytes(
        &include_bytes!(concat!(env!("OUT_DIR"), "/opt.spv"))[..],
        &mut loader,
    )
    .unwrap();
}
