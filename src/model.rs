use obj::Obj;

use crate::{
    geometry::{Vec2i, Vec2f, Vec3f},
    tga::{TGAImage},
};

/*
    OBJ Model Wrapper Class

    Adds diffuse map capabilities to OBJ struct provided
    by the obj crate.
*/

pub struct Model {
    obj: Obj,
    diffusemap: TGAImage,
    normalmap: TGAImage,
    specularmap: TGAImage
}

impl Model {
    pub fn new_args(filename: &str) -> Self {
        let mut diffusemap = TGAImage::new();
        let mut normalmap = TGAImage::new();
        let mut specularmap = TGAImage::new();
        let model = Obj::load(filename).unwrap();
        Model::load_texture(&String::from(filename), "_diffuse.tga", &mut diffusemap);
        Model::load_texture(&String::from(filename), "_nm.tga", &mut normalmap);
        Model::load_texture(&String::from(filename), "_spec.tga", &mut specularmap);
        Self {
            obj: model,
            diffusemap: diffusemap,
            normalmap: normalmap,
            specularmap: specularmap,
        }
    }

    pub fn get_model(&self) -> &Obj {
        &self.obj
    }

    pub fn load_texture(filename: &String, suffix: &str, img: &mut TGAImage) {
        let dot = filename.rfind(".");
        if let Some(idx) = dot {
            let mut texfile = String::from(&filename.clone()[0..idx]);
            texfile.push_str(suffix);
            println!(
                "Texture file {texfile} loading {}",
                if let Ok(_) = img.read_tga_file(&texfile) {
                    "ok"
                } else {
                    "failed"
                }
            );
            //img.flip_vertically();
        }
    }

    pub fn nfaces(&self) -> usize {
        self.obj.data.objects[0].groups[0].polys.len()
    }
    pub fn vert(&self, iface: i32, nthvert: i32) -> Vec3f {
        let idx = self.obj.data.objects[0].groups[0].polys[iface as usize].0[nthvert as usize].0;
        Vec3f::from_slice(&self.obj.data.position[idx])
    }
    pub fn uv(&self, iface: i32, nthvert: i32) -> Vec2f {
        let idx = self.obj.data.objects[0].groups[0].polys[iface as usize].0[nthvert as usize]
            .1
            .unwrap();

        Vec2f::new_args(self.obj.data.texture[idx][0], 1.-self.obj.data.texture[idx][1])
    }
    pub fn norm(&self, iface: i32, nvert: i32) -> Vec3f {
        let idx = self.obj.data.objects[0].groups[0].polys[iface as usize].0[nvert as usize]
            .2
            .unwrap();
        let norm = self.obj.data.normal[idx];
        Vec3f::new_args(norm[0], norm[1], norm[2]).normalize().to_owned()
    }
    pub fn normal(&self, uvf: &Vec2f) -> Vec3f {
        let uv = Vec2i::new_args((uvf[0]*self.normalmap.get_width() as f32) as i32, (uvf[1]*self.normalmap.get_height() as f32) as i32);
        let c = self.normalmap.get(uv[0], uv[1]);
        let mut res = Vec3f::new();
        for i in 0..3 {
            res[2-i] = c[i] as f32/255.*2.-1.;
        }
        res
    }

    pub fn diffuse(&self) -> &TGAImage {
        &self.diffusemap
    }
    pub fn specular(&self) -> &TGAImage {
        &self.specularmap
    }
}
