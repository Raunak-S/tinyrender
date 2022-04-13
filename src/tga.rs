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
pub struct TGAColor {
    pub bgra: [u8; 4],
    pub bytespp: u8,
}

impl TGAColor {
    pub fn new() -> Self {
        Self {
            bgra: [0, 0, 0, 0],
            bytespp: 0,
        }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            bgra: [b, g, r, a],
            bytespp: 4,
        }
    }

    pub fn new_raw(p: &[u8], bpp: u8) -> Self {
        let mut bgra = [0; 4];
        for i in 0..bpp as usize {
            bgra[i] = p[i];
        }
        Self {
            bgra: bgra,
            bytespp: bpp,
        }
    }
}

impl Index<usize> for TGAColor {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bgra[index]
    }
}

impl IndexMut<usize> for TGAColor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bgra[index]
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
        let mut colorbuffer = TGAColor::new();
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
                for _ in 0..chunkheader {
                    if let Err(e) = f.read_exact(&mut colorbuffer.bgra[0..self.bytespp as usize]) {
                        eprintln!("Error occured while reading the data, {e:?}");
                        return false;
                    }
                    for t in 0..self.bytespp {
                        self.data.as_mut().unwrap()[currentbyte as usize] = colorbuffer.bgra[t as usize];
                        currentbyte += 1;
                    }
                    currentpixel += 1;
                    if currentpixel > pixelcount {
                        eprintln!("Too many pixels read");
                        return false;
                    }
                }
            } else {
                chunkheader -= 127;
                if let Err(e) = f.read_exact(&mut colorbuffer.bgra[0..self.bytespp as usize]) {
                    eprintln!("Error occured while reading the data, {e:?}");
                    return false;
                }
                for _ in 0..chunkheader {
                    for t in 0..self.bytespp {
                        self.data.as_mut().unwrap()[currentbyte as usize] = colorbuffer.bgra[t as usize];
                        currentbyte += 1;
                    }
                    currentpixel += 1;
                    if currentpixel > pixelcount {
                        eprintln!("Too many pixels read");
                        return false;
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

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn set(&mut self, x: i32, y: i32, c: &TGAColor) {
        if self.data.is_none() || x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let index = ((x + y * self.width) * self.bytespp) as usize;   
        let ptr = &mut self.data.as_mut().unwrap().as_mut_slice()
        [index..index + self.bytespp as usize];
        ptr.copy_from_slice(&c.bgra[0..self.bytespp as usize]);
    }

    pub fn get(&self, x: i32, y: i32) -> TGAColor {
        if self.data.is_none() || x < 0 || y < 0 || x >= self.width || y >= self.height {
            return TGAColor::new();
        }

        let index = ((x + y * self.width) * self.bytespp) as usize;
        return TGAColor::new_raw(
            &self.data.as_ref().unwrap().as_slice()[index..index + self.bytespp as usize],
            self.bytespp as u8,
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
