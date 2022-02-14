mod tga;
mod geometry;
mod model;
use crate::{tga::*, geometry::*};
use model::Model;
use obj::Obj;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const DEPTH: i32 = 255;
const INPUT: &str = "obj/african_head.obj";
const OUTPUT: &str = "obj/output.tga";

pub fn m2v(m: Matrix) -> Vec3f {
    Vec3f::new_args(m[0][0]/m[3][0], m[1][0]/m[3][0], m[2][0]/m[3][0])
}

pub fn v2m(v: Vec3f) -> Matrix {
    let mut m = Matrix::new(Some(4), Some(1));
    m[0][0] = v.x;
    m[1][0] = v.y;
    m[2][0] = v.z;
    m[3][0] = 1.;
    m
}

pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> Matrix {
    let mut m = Matrix::identity(4);
    m[0][3] = x as f32+w as f32/2.;
    m[1][3] = y as f32+h as f32/2.;
    m[2][3] = DEPTH as f32/2.;

    m[0][0] = w as f32/2.;
    m[1][1] = h as f32/2.;
    m[2][2] = DEPTH as f32/2.;
    m
}

fn main() {
    let light_dir = Vec3f::new_args(0.0, 0.0, -1.0);
    let camera = Vec3f::new_args(0.0, 0.0, 3.0);

    let model_wrapper = Model::new_args(INPUT);
    let model = model_wrapper.get_model();
    let mut zbuffer = [i32::MIN; (WIDTH*HEIGHT) as usize];

    {
        let mut Projection = Matrix::identity(4);
        let ViewPort = viewport(WIDTH/8, HEIGHT/8, WIDTH*3/4, HEIGHT*3/4);
        Projection[3][2] = -1./camera.z;

        let mut image = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.RGB as i32);
        for i in 0..model.data.objects[0].groups[0].polys.len() {
            let face = &model.data.objects[0].groups[0].polys[i];
            let mut screen_coords = [Vec3i::new(); 3];
            let mut world_coords = [Vec3f::new(); 3];
            for j in 0..3 {
                let v = model.data.position[face.0[j].0];
                let v = Vec3f::new_args(v[0], v[1], v[2]);
                // intermediate Vec to convert from Vec3f -> Vec3i
                let coord_conv_vec = m2v(ViewPort.clone()*Projection.clone()*v2m(v));
                screen_coords[j] = Vec3i::new_args(coord_conv_vec.x as i32, coord_conv_vec.y as i32, coord_conv_vec.z as i32);
                world_coords[j] = v;
            }
            let mut n = (world_coords[2]-world_coords[0])^(world_coords[1]-world_coords[0]);
            n.normalize();
            let intensity = n*light_dir;
            if intensity>0.0 {
                let mut uv = [Vec2i::new(); 3];
                for k in 0..3 {
                    uv[k] = model_wrapper.uv(i as i32, k as i32);
                }
                triangle(screen_coords[0], screen_coords[1], screen_coords[2], uv[0], uv[1], uv[2], &mut image, intensity, &mut zbuffer, &model_wrapper);
            }
        }
        
        image.flip_vertically();
        image.write_tga_file(OUTPUT, false).unwrap();
    }
}