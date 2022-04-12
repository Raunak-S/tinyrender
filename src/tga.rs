use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::{self, Error};
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug)]
pub enum ColorType {
    RGBA(RGBA),
    Raw([u8; 4]),
    Val(u32),
}

#[derive(Debug)]
pub struct TGAColor {
    pub color_type: ColorType,
    pub bytespp: i32,
}

impl TGAColor {
    pub fn new() -> Self {
        Self {
            color_type: ColorType::Val(0),
            bytespp: 1,
        }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            color_type: ColorType::RGBA(RGBA {
                r: r,
                g: g,
                b: b,
                a: a,
            }),
            bytespp: 4,
        }
    }

    pub fn new_val(v: u32, bpp: i32) -> Self {
        Self {
            color_type: ColorType::Val(v),
            bytespp: bpp,
        }
    }

    pub fn new_raw(p: &[u8], bpp: i32) -> Self {
        let mut raw = [0; 4];
        for i in 0..bpp as usize {
            raw[i] = p[i];
        }
        Self {
            color_type: ColorType::Raw(raw),
            bytespp: bpp,
        }
    }
}

impl Index<usize> for TGAColor {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match self.color_type {
            ColorType::Raw(ref arr) => &arr[index],
            _ => panic!(),
        }
    }
}

impl IndexMut<usize> for TGAColor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.color_type {
            ColorType::Raw(ref mut arr) => &mut arr[index],
            ColorType::RGBA(ref mut rgba) => {
                if index == 0 {
                    return &mut rgba.b;
                } else {
                    if index == 1 {
                        return &mut rgba.g;
                    } else {
                        if index == 2 {
                            return &mut rgba.r;
                        } else {
                            return &mut rgba.a;
                        }
                    }
                }
            }
            _ => {
                eprintln!("{:?}", self.color_type);
                panic!()
            }
        }
    }
}

impl Mul<f32> for TGAColor {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let rhs = if rhs > 1. {
            1.
        } else {
            if rhs < 0. {
                0.
            } else {
                rhs
            }
        };
        match self.color_type {
            ColorType::RGBA(rgba) => TGAColor::new_rgba(
                (rgba.r as f32 * rhs) as u8,
                (rgba.g as f32 * rhs) as u8,
                (rgba.b as f32 * rhs) as u8,
                (rgba.a as f32 * rhs) as u8,
            ),
            _ => TGAColor::new(),
        }
    }
}

pub struct Format {
    pub Grayscale: u8,
    pub RGB: u8,
    pub RGBA: u8,
}

pub const TGAFormat: Format = Format {
    Grayscale: 1,
    RGB: 3,
    RGBA: 4,
};

pub struct TGAImage {
    data: Option<Box<Vec<u8>>>,
    width: i32,
    height: i32,
    bytespp: i32,
}

impl TGAImage {
    pub fn new() -> Self {
        Self {
            data: None,
            width: 0,
            height: 0,
            bytespp: 0,
        }
    }

    pub fn new_dimensions(w: i32, h: i32, bpp: i32) -> Self {
        let nbytes = w * h * bpp;
        let data = vec![0; nbytes as usize];
        Self {
            data: Some(Box::new(data)),
            width: w,
            height: h,
            bytespp: bpp,
        }
    }

    pub fn read_tga_file(&mut self, filename: &str) -> io::Result<()> {
        let mut f = BufReader::new(File::open(filename)?);

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

        let nbytes = self.bytespp * self.width * self.height;
        self.data = Some(Box::new(vec![0; nbytes as usize]));

        if header.data_type_code == 3 || header.data_type_code == 2 {
            todo!()
        } else if header.data_type_code == 10 || header.data_type_code == 11 {
            if !self.load_rle_data(&mut f) {
                eprintln!("Error occured while reading the data");
                return Err(Error::last_os_error());
            }
        } else {
            todo!()
        }

        if header.image_descriptor & 0x20 == 0 {
            self.flip_vertically();
        }
        if header.image_descriptor & 0x10 != 0 {
            self.flip_horizontally();
        }

        Ok(())
    }

    pub fn load_rle_data(&mut self, f: &mut BufReader<File>) -> bool {
        let pixelcount = (self.width * self.height) as u32;
        let mut currentpixel = 0 as u32;
        let mut currentbyte = 0 as u32;
        let colorbuffer = TGAColor::new_raw(&[0; 4], self.bytespp);
        loop {
            let mut chunkheader;
            match f.read_u8() {
                Ok(val) => chunkheader = val,
                Err(e) => {
                    eprintln!("Error occured while reading the data, {e:?}");
                    return false;
                }
            }
            if chunkheader < 128 {
                chunkheader += 1;
                for i in 0..chunkheader {
                    if let ColorType::Raw(mut arr) = colorbuffer.color_type {
                        if let Err(e) = f.read_exact(&mut arr[0..self.bytespp as usize]) {
                            eprintln!("Error occured while reading the data, {e:?}");
                            return false;
                        }
                        for t in 0..self.bytespp {
                            self.data.as_mut().unwrap()[currentbyte as usize] = arr[t as usize];
                            currentbyte += 1;
                        }
                        currentpixel += 1;
                        if currentpixel > pixelcount {
                            eprintln!("Too many pixels read");
                            return false;
                        }
                    }
                }
            } else {
                chunkheader -= 127;
                if let ColorType::Raw(mut arr) = colorbuffer.color_type {
                    if let Err(e) = f.read_exact(&mut arr[0..self.bytespp as usize]) {
                        eprintln!("Error occured while reading the data, {e:?}");
                        return false;
                    }
                    for _ in 0..chunkheader {
                        for t in 0..self.bytespp {
                            self.data.as_mut().unwrap()[currentbyte as usize] = arr[t as usize];
                            currentbyte += 1;
                        }
                        currentpixel += 1;
                        if currentpixel > pixelcount {
                            eprintln!("Too many pixels read");
                            return false;
                        }
                    }
                }
            }
            if !(currentpixel < pixelcount) {
                break;
            }
        }
        true
    }

    pub fn write_tga_file(&self, filename: &str, rle: bool) -> io::Result<()> {
        let developer_area_ref: [u8; 4] = [0, 0, 0, 0];
        let extension_area_ref: [u8; 4] = [0, 0, 0, 0];

        // let mut footer: &[char] = &[
        //     'T', 'R', 'U', 'E', 'V', 'I', 'S', 'I', 'O', 'N', '-', 'X', 'F', 'I', 'L', 'E', '.',
        //     '\0',
        // ];

        let mut out = File::create(filename)?;
        let mut header = TGAHeader::new();
        header.bits_per_pixel = (self.bytespp as u8) << 3;
        header.width = self.width as u16;
        header.height = self.height as u16;
        header.data_type_code = if self.bytespp == TGAFormat.Grayscale as i32 {
            if rle {
                11
            } else {
                3
            }
        } else {
            if rle {
                10
            } else {
                2
            }
        };
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

    pub fn set(&mut self, x: i32, y: i32, c: &TGAColor) -> bool {
        if self.data.is_none() || x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }
        let index = ((x + y * self.width) * self.bytespp) as usize;
        match &c.color_type {
            ColorType::RGBA(buf) => {
                let ptr = &mut self.data.as_mut().unwrap().as_mut_slice()
                    [index..index + self.bytespp as usize];
                ptr.copy_from_slice(&[buf.b, buf.g, buf.r]);
            }
            ColorType::Val(val) => {
                let ptr = &mut self.data.as_mut().unwrap().as_mut_slice()
                    [index..index + self.bytespp as usize];
                ptr.copy_from_slice(&[*val as u8]);
            }
            _ => println!("Nothing here"),
        }
        true
    }

    pub fn get(&self, x: i32, y: i32) -> TGAColor {
        if self.data.is_none() || x < 0 || y < 0 || x >= self.width || y >= self.height {
            return TGAColor::new();
        }

        let index = ((x + y * self.width) * self.bytespp) as usize;
        return TGAColor::new_raw(
            &self.data.as_ref().unwrap().as_slice()[index..index + self.bytespp as usize],
            self.bytespp,
        );
    }

    pub fn flip_vertically(&mut self) -> bool {
        if self.data.is_none() {
            return false;
        }
        let bytes_per_line = (self.width * self.bytespp) as usize;
        let mut line = vec![0; bytes_per_line];
        let half = self.height >> 1;
        for i in 0..half as usize {
            // TODO: change self.{height, width, length, ...} to usize
            let l1 = (i * bytes_per_line) as usize;
            let l2 = (self.height as usize - 1 - i) * bytes_per_line;
            line.as_mut_slice()
                .copy_from_slice(&self.data.as_ref().unwrap().as_slice()[l1..l1 + bytes_per_line]);
            let line1 = self.data.as_mut().unwrap().as_mut_slice();
            line1.copy_within(l2..l2 + bytes_per_line, l1);
            line1[l2..l2 + bytes_per_line].copy_from_slice(line.as_slice());
        }
        true
    }

    pub fn flip_horizontally(&mut self) -> bool {
        todo!()
    }
}
