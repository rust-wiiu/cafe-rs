use crate::prelude::*;
use sys::coreinit::font;

pub use font::Font;

pub fn system_font(font: Font) -> Result<&'static [u8], ()> {
    let mut data = std::ptr::null();
    let mut len = 0;
    unsafe {
        if font::get_shared_font(font, 0, &mut data, &mut len) == 0 {
            Err(())
        } else {
            Ok(std::slice::from_raw_parts(data, len))
        }
    }
}
