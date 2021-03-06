#[allow(deref_nullptr)]
pub mod bindings {
    use ispc_rt::ispc_module;

    ispc_module!(kernel);
    ispc_module!(kernel_astc);
}

pub mod astc;
pub mod bc1;
pub mod bc3;
pub mod bc4;
pub mod bc5;
pub mod bc6h;
pub mod bc7;
pub mod etc1;

#[derive(Debug, Copy, Clone)]
pub struct RgbaSurface<'a> {
    pub data: &'a [u8],
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

pub fn cal_block_count(
    image_width: u32,
    image_height: u32,
    block_width: u32,
    block_height: u32,
) -> u32 {
    ((image_width + block_width - 1) / block_width)
        * ((image_height + block_height - 1) / block_height)
}
