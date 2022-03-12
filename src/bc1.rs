use more_asserts::assert_ge;

use crate::bindings::kernel;
use crate::{cal_block_count, RgbaSurface};

#[inline(always)]
pub fn calc_output_size(width: u32, height: u32) -> usize {
    // BC1 uses 8 bytes to store each 4×4 block, giving it an average data rate of 0.5 bytes per pixel.
    let block_count = cal_block_count(width, height, 4, 4) as usize;
    block_count * 8
}

pub fn compress_blocks(surface: &RgbaSurface) -> Vec<u8> {
    let output_size = calc_output_size(surface.width, surface.height);
    let mut output = vec![0u8; output_size];
    compress_blocks_into(surface, &mut output);
    output
}

pub fn compress_blocks_into(surface: &RgbaSurface, blocks: &mut [u8]) {
    assert_ge!(
        blocks.len(),
        calc_output_size(surface.width, surface.height)
    );
    let mut surface = kernel::rgba_surface {
        width: surface.width as i32,
        height: surface.height as i32,
        stride: surface.stride as i32,
        ptr: surface.data.as_ptr() as *mut u8,
    };

    unsafe {
        kernel::CompressBlocksBC1_ispc(&mut surface, blocks.as_mut_ptr());
    }
}
