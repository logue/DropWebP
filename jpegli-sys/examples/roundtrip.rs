use jpegli_sys::*;
use std::mem;

fn main() {
    unsafe {
        let mut cinfo: jpegli_compress_struct = mem::zeroed();
        let mut jerr: jpegli_error_mgr = mem::zeroed();
        cinfo.common.err = jpegli_std_error(&mut jerr);
        jpegli_CreateCompress(&mut cinfo, JPEG_LIB_VERSION, mem::size_of::<jpegli_compress_struct>());

        let mut outbuf: *mut u8 = std::ptr::null_mut();
        let mut outsize: std::os::raw::c_ulong = 0;
        jpegli_mem_dest(&mut cinfo, &mut outbuf, &mut outsize);

        cinfo.image_width = 32;
        cinfo.image_height = 32;
        cinfo.input_components = 3;
        cinfo.in_color_space = J_COLOR_SPACE::JCS_RGB;
        jpegli_set_defaults(&mut cinfo);
        jpegli_set_quality(&mut cinfo, 90, 1);
        jpegli_simple_progression(&mut cinfo);
        jpegli_start_compress(&mut cinfo, 1);

        let row = vec![128u8; 32 * 3];
        while cinfo.next_scanline < cinfo.image_height {
            let ptrs = [row.as_ptr()];
            jpegli_write_scanlines(&mut cinfo, ptrs.as_ptr(), 1);
        }
        jpegli_finish_compress(&mut cinfo);

        let out = std::slice::from_raw_parts(outbuf, outsize as usize);
        println!("Encoded {} bytes, header: {:02x} {:02x}", out.len(), out[0], out[1]);
        assert!(out.len() > 100);
        assert_eq!(&out[0..2], &[0xFF, 0xD8]);
        println!("SUCCESS");
    }
}
