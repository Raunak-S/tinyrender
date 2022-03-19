mod geometry;
mod model;
mod our_gl;
mod tga;

use crate::{geometry::*, our_gl::*, tga::*};
use model::*;
use obj::Obj;

const INPUT: &str = "obj/african_head.obj";
const OUTPUT: &str = "obj/output.tga";

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

struct GouraudShader {
    varying_intensity: Vec3f,
}

impl Shader for GouraudShader {
    fn vertex(&mut self, iface: i32, nthvert: i32, model: &Obj, light_dir: Vec3f, view_bundle: &ViewBundle) -> Vec4f {
        let idx = model.data.objects[0].groups[0].polys[iface as usize].0[nthvert as usize]
        .0;
        let mut gl_Vertex = embed(&Vec3f::from_slice(&model.data.position[idx]));
        gl_Vertex = view_bundle.ViewPort.clone()*view_bundle.ModelView.clone()*view_bundle.Projection.clone()*gl_Vertex;
        self.varying_intensity[nthvert as usize] = 0f32.max(Vec3f::from_slice(&model.data.normal[idx]).normalize().to_owned()*light_dir);
        gl_Vertex
    }

    fn fragment(&self, bar: Vec3f) -> (bool, TGAColor) {
        let intensity = self.varying_intensity*bar;
        (false, TGAColor::new_rgba(255, 255, 255, 255)*intensity)
    }
}

fn main() {
    let model_wrapper = Model::new_args(INPUT);
    let model = model_wrapper.get_model();

    let light_dir = Vec3f::new_args(1., 1., 1.).normalize().to_owned();
    let eye = Vec3f::new_args(1., 1., 3.);
    let center = Vec3f::new_args(0., 0., 0.);
    let up = Vec3f::new_args(0., 1., 0.);

    let ModelView = lookat(eye, center, up);
    let ViewPort = viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4);
    let Projection = projection(-1. / (eye - center).norm());
    let view_bundle = ViewBundle {ModelView, ViewPort, Projection};

    let mut image = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.RGB as i32);
    let mut zbuffer = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.Grayscale as i32);

    let mut shader = GouraudShader { varying_intensity: Vec3f::new() };
    for i in 0..model.data.objects[0].groups[0].polys.len() {
        let mut screen_coords = [Vec4f::new(); 3];
        for j in 0..3 {
            screen_coords[j as usize] = shader.vertex(i as i32, j, &model, light_dir, &view_bundle);
        }
        triangle(&screen_coords, &shader, &mut image, &mut zbuffer);
    }

    image.flip_vertically();
    zbuffer.flip_vertically();
    image.write_tga_file(OUTPUT, false).unwrap();
    zbuffer.write_tga_file("zbuffer.tga", false).unwrap();
}