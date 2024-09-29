use std::io::{self, Write};

pub fn write_color(out: &mut dyn Write, pixel_color: image::Rgba<u8>) -> io::Result<()> {
    let r = pixel_color.data[0];
    let g = pixel_color.data[1];
    let b = pixel_color.data[2];

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", r, g, b)
}
