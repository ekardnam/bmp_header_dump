/*
 *   bmp_header_dump - Dump BMP file headers
 *   Copyright (C) 2019 Luca "ekardnam" Bertozzi <ekardnam@autistici.org>
 *
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *   along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate argparse;
extern crate byteorder;

use std::{
    string::String,
    fs::File,
    io::{
        Result,
        Cursor,
        Read
    }
};
use argparse::{ArgumentParser, Store};
use byteorder::{ReadBytesExt, LittleEndian};


fn print_headers(filename: String) -> Result<()> {
    let mut file = File::open(filename)?;
    const BMP_HEADER_SIZE: usize = 14 + 4; // 4 for reading the dib header size
    let mut header = [0; BMP_HEADER_SIZE];
    file.read(&mut header)?;
    let mut cur = Cursor::new(header);

    let magic_bytes = String::from_utf8(header[0..2].to_vec()).unwrap();
    let filetype = match magic_bytes.as_ref() {
        "BM" => "Bitmap Windows 3.1x, 95, NT".to_string(),
        "BA" => "OS/2 struct bitmap array".to_string(),
        "CI" => "OS/2 struct color icon".to_string(),
        "CP" => "OS/2 const color pointer".to_string(),
        "IC" => "OS/2 struct icon".to_string(),
        "PT" => "OS/2 pointer".to_string(),
        &_   => "Unrecognized type".to_string(),
    };

    cur.set_position(2);
    let size = cur.read_u32::<LittleEndian>().unwrap();
    let reserved = cur.read_u32::<LittleEndian>().unwrap();
    let image_offset = cur.read_u32::<LittleEndian>().unwrap();
    let dib_header_size = cur.read_u32::<LittleEndian>().unwrap();

    let mut dib_header_buf = vec![0u8; (dib_header_size - 4) as usize];
    file.read_exact(&mut dib_header_buf)?;

    let mut dib_cur = Cursor::new(dib_header_buf);

    let width = dib_cur.read_u32::<LittleEndian>().unwrap();
    let height = dib_cur.read_u32::<LittleEndian>().unwrap();
    let color_planes_count = dib_cur.read_u16::<LittleEndian>().unwrap();
    let color_depth = dib_cur.read_u16::<LittleEndian>().unwrap();
    let compression = dib_cur.read_u32::<LittleEndian>().unwrap();
    let compression_method = match compression {
        0 => "BI_RGB".to_string(),
        1 => "BI_RLE8".to_string(),
        2 => "BI_RLE4".to_string(),
        3 => "BI_BITFIELDS".to_string(),
        4 => "BI_JPEG".to_string(),
        5 => "BI_PNG".to_string(),
        6 => "BI_ALPHABITFIELDS".to_string(),
        7 => "BI_CMYK".to_string(),
        8 => "BI_CMYKRLE8".to_string(),
        9 => "BI_CMYKRLE4".to_string(),
        _ => "Unrecgnized compression method".to_string(),
    };
    let image_size = dib_cur.read_u32::<LittleEndian>().unwrap();
    let hor_res = dib_cur.read_u32::<LittleEndian>().unwrap();
    let ver_res = dib_cur.read_u32::<LittleEndian>().unwrap();
    let color_count = dib_cur.read_u32::<LittleEndian>().unwrap();
    let important_color_count = dib_cur.read_u32::<LittleEndian>().unwrap();

    println!("");
    println!("|\tBMP header\t|");
    println!("Image type: {}", filetype);
    println!("Image size: {}", size);
    println!("Reserved bytes: {}", reserved);
    println!("Image offset: 0x{:X}", image_offset);
    println!("");
    println!("|\tDIB header\t|");
    println!("DIB header size: {}", dib_header_size);
    println!("Image width: {}", width);
    println!("Image height: {}", height);
    println!("Color planes count: {}", color_planes_count);
    println!("Color depth: {}", color_depth);
    println!("Compression method: {}", compression_method);
    println!("Image size: {}", image_size);
    println!("Horizontal resoution: {}", hor_res);
    println!("Vertical resolution: {}", ver_res);
    println!("Color count: {}", color_count);
    println!("Important color count: {}", important_color_count);
    println!("");

    Ok(())
}

fn main() -> Result<()> {
    let mut file = "image.bmp".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Dump BMP file headers");
        ap.refer(&mut file)
            .add_option(&["-f", "--file"], Store,
            "BMP file name");
        ap.parse_args_or_exit();
    }

    print_headers(file)
}
