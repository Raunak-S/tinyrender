use obj::Obj;

use crate::{geometry::{Vec2i}, tga::{TGAImage, TGAColor}};

/*
    OBJ Model Wrapper Class

    Adds diffuse map capabilities to OBJ struct provided
    by the obj crate.
*/

pub struct Model {
    obj: Obj,
    diffusemap: TGAImage,
}

impl Model {
    pub fn new_args(filename: &str) -> Self {
        let mut diffusemap = TGAImage::new();
        let model = Obj::load(filename).unwrap();
        Model::load_texture(&String::from(filename), "_diffuse.tga", &mut diffusemap);
        Self { obj: model, diffusemap: diffusemap }
    }

    pub fn get_model(&self) -> &Obj {
        &self.obj
    }

    pub fn load_texture(filename: &String, suffix: &str, img: &mut TGAImage) {
        let dot = filename.rfind(".");
        if let Some(idx) = dot {
            let mut texfile = String::from(&filename.clone()[0..idx]);
            texfile.push_str(suffix);
            println!("Texture file {texfile} loading {}", if let Ok(_) = img.read_tga_file(&texfile) { "ok" } else { "failed" });
            img.flip_vertically();
        }
    }

    pub fn diffuse(&self, uv: Vec2i) -> TGAColor {
        self.diffusemap.get(uv.x, uv.y)
    }

    pub fn uv(&self, iface: i32, nvert: i32) -> Vec2i {
        let idx = self.obj.data.objects[0].groups[0].polys[iface as usize].0[nvert as usize].1.unwrap();
        Vec2i::new_args((self.obj.data.texture[idx][0]*self.diffusemap.get_width() as f32) as i32, (self.obj.data.texture[idx][1]*self.diffusemap.get_height() as f32) as i32)
    }
}