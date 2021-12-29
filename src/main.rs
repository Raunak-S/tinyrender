use crate::ColorType::*;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use byteorder::{ReadBytesExt, LittleEndian};

struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    data_type_code: u8,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl TGAHeader {
    pub fn new() -> Self {
        Self {
            id_length: 0,
            color_map_type: 0,
            data_type_code: 0,
            color_map_origin: 0,
            color_map_length: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 0,
            width: 0,
            height: 0,
            bits_per_pixel: 0,
            image_descriptor: 0,
        }
    }
}


struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

enum ColorType {
    RGBA(RGBA),
    Raw([u8; 4]),
    Val(u32),
}

struct TGAColor {
    color_type: ColorType,
    bytespp: i32,
}

impl TGAColor {
    pub fn new() -> Self {
        Self { color_type: Val(0), bytespp: 1, }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { color_type: RGBA(RGBA { r: r, g: g, b: b, a: a, }), bytespp: 4, }
    }

    pub fn new_val(v: u32, bpp: i32) -> Self {
        Self { color_type: Val(v), bytespp: bpp, }
    }

    pub fn new_raw(p: &[u8], bpp: i32) -> Self {
        let mut raw = [0; 4];
        for i in 0..bpp as usize {
            raw[i] = p[i];
        }
        Self { color_type: Raw(raw), bytespp: bpp, }
    }
}



struct Format {
    Grayscale: u8,
    RGB: u8,
    RGBA: u8,
}

const TGAFormat: Format = Format {
    Grayscale: 1,
    RGB: 3,
    RGBA: 4,
};

struct TGAImage {
    data: Option<Box<Vec<u8>>>,
    width: i32,
    height: i32,
    bytespp: i32,
}

impl TGAImage {
    pub fn new() -> Self {
        Self { data: None, width: 0, height: 0, bytespp: 0, }
    }

    pub fn new_dimensions(w: i32, h: i32, bpp: i32) -> Self {
        let nbytes = w*h*bpp;
        let data = vec![0; nbytes as usize];
        Self { data: Some(Box::new(data)), width: w, height: h, bytespp: bpp, }
    }

    pub fn read_tga_file(&mut self, filename: &str) -> io::Result<()> {
        let mut f = File::open(filename)?;

        let header = TGAHeader {
            id_length: f.read_u8()?,
            color_map_type: f.read_u8()?,
            data_type_code: f.read_u8()?,
            color_map_origin: f.read_u16::<LittleEndian>()?,
            color_map_length: f.read_u16::<LittleEndian>()?,
            color_map_depth: f.read_u8()?,
            x_origin: f.read_u16::<LittleEndian>()?,
            y_origin: f.read_u16::<LittleEndian>()?,
            width: f.read_u16::<LittleEndian>()?,
            height: f.read_u16::<LittleEndian>()?,
            bits_per_pixel: f.read_u8()?,
            image_descriptor: f.read_u8()?,
        };

        self.width = header.width as i32;
        self.height = header.height as i32;
        self.bytespp = header.bits_per_pixel as i32 >> 3;

        let nbytes = self.bytespp*self.width*self.height;
        self.data = Some(Box::new(Vec::with_capacity(nbytes as usize)));

        Ok(())
    }

    pub fn write_tga_file(&self, filename: &str, rle: bool) -> io::Result<()> {
        let developer_area_ref: [u8; 4] = [0, 0, 0, 0];
        let extension_area_ref: [u8; 4] = [0, 0, 0, 0];

        let mut footer: &[char] = &['T','R','U','E','V','I','S','I','O','N','-','X','F','I','L','E','.','\0'];

        let mut out = File::create(filename)?;
        let mut header = TGAHeader::new();
        header.bits_per_pixel = (self.bytespp as u8) << 3;
        header.width = self.width as u16;
        header.height = self.height as u16;
        header.data_type_code = if self.bytespp==TGAFormat.Grayscale as i32 { if rle {11} else {3} } else { if rle {10} else {2} };
        header.image_descriptor = 0x20;

        out.write(&header.id_length.to_le_bytes())?;
        out.write(&header.color_map_type.to_le_bytes())?;
        out.write(&header.data_type_code.to_le_bytes())?;
        out.write(&header.color_map_origin.to_le_bytes())?;
        out.write(&header.color_map_length.to_le_bytes())?;
        out.write(&header.color_map_depth.to_le_bytes())?;
        out.write(&header.x_origin.to_le_bytes())?;
        out.write(&header.y_origin.to_le_bytes())?;
        out.write(&header.width.to_le_bytes())?;
        out.write(&header.height.to_le_bytes())?;
        out.write(&header.bits_per_pixel.to_le_bytes())?;
        out.write(&header.image_descriptor.to_le_bytes())?;

        out.write(self.data.as_ref().unwrap().as_slice())?;

        Ok(())
    }

    pub fn get_bytespp(&self) -> i32 {
        self.bytespp
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn set(&mut self, x: i32, y:i32, c: &TGAColor) -> bool {
        if self.data.is_none() || x<0 || y<0 || x>=self.width || y>=self.height { return false; }
        let index = ((x+y*self.width)*self.bytespp) as usize;
        match &c.color_type {
            RGBA(buf) => {
                let ptr = &mut self.data.as_mut().unwrap().as_mut_slice()[index..index+self.bytespp as usize];
                ptr.copy_from_slice(&[buf.b, buf.g, buf.r]);
            },
            _ => println!("Nothing here")
        }
        true
    }

    pub fn get(&self, x: i32, y: i32) -> TGAColor {
        if !self.data.is_none() || x<0 || y<0 || x>=self.width || y>=self.height { return TGAColor::new(); }

        let index = ((x+y*self.width)*self.bytespp) as usize;
        return TGAColor::new_raw(&self.data.as_ref().unwrap().as_slice()[index..], self.bytespp);
    }

    pub fn flip_vertically(&mut self) -> bool {
        if self.data.is_none() { return false; }
        let bytes_per_line = (self.width*self.bytespp) as usize;
        let mut line = vec![0; bytes_per_line];
        let half = self.height>>1;
        for i in 0..half as usize {
            // TODO: change self.{height, width, length, ...} to usize
            let l1 = (i*bytes_per_line) as usize;
            let l2 = (self.height as usize-1-i)*bytes_per_line;
            line.as_mut_slice().copy_from_slice(&self.data.as_ref().unwrap().as_slice()[l1..l1+bytes_per_line]);
            let line1 = self.data.as_mut().unwrap().as_mut_slice();
            line1.copy_within(l2..l2+bytes_per_line, l1);
            line1[l2..l2+bytes_per_line].copy_from_slice(line.as_slice());
        }
        true
    }

}
    

fn main() {
    let white = TGAColor::new_rgba(255, 255, 255, 255);
    let red = TGAColor::new_rgba(255, 0, 0, 255);

    let mut image = TGAImage::new_dimensions(100, 100, TGAFormat.RGB as i32);
    image.set(52, 41, &red);
    image.set(52, 59, &white);
    image.flip_vertically();
    image.write_tga_file("/mnt/c/Users/shresr/Desktop/output.tga", false).unwrap();
}