mod tga;
mod geometry;
use crate::{tga::*, geometry::*};
use obj::Obj;

pub fn check_expected() {
    let white = TGAColor::new_rgba(255, 255, 255, 255);
    let red = TGAColor::new_rgba(255, 0, 0, 255);

    let mut image = TGAImage::new_dimensions(100, 100, TGAFormat.RGB as i32);
    image.set(52, 41, &red);
    image.flip_vertically();
    image.write_tga_file("/mnt/c/Users/shresr/Desktop/output.tga", false).unwrap();
}

pub fn check_expected1() {
    let model = Obj::load("/mnt/c/Users/shresr/Desktop/african_head.obj").unwrap();

    let mut image = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.RGB as i32);
    for i in 0..model.data.objects[0].groups[0].polys.len() {
        let face = &model.data.objects[0].groups[0].polys[i];
        for j in 0..3 {
            let v0 = model.data.position[face.0[j].0];
            let v1 = model.data.position[face.0[(j+1)%3].0];
            let x0 = ((v0[0]+1.)*(WIDTH as f32)/2.) as i32;
            let y0 = ((v0[1]+1.)*(HEIGHT as f32)/2.) as i32;
            let x1 = ((v1[0]+1.)*(WIDTH as f32)/2.) as i32;
            let y1 = ((v1[1]+1.)*(HEIGHT as f32)/2.) as i32;
            line(x0, y0, x1, y1, &mut image, &WHITE);
        }
    }
    image.flip_vertically();
    image.write_tga_file("/mnt/c/Users/shresr/Desktop/output.tga", false).unwrap();
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
    let model = Obj::load("/mnt/c/Users/shresr/Desktop/african_head.obj").unwrap();

    let mut image = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.RGB as i32);
    let light_dir = Vec3f::new_args(0.0,0.0,-1.0);
    for i in 0..model.data.objects[0].groups[0].polys.len() {
        let face = &model.data.objects[0].groups[0].polys[i];
        let mut screen_coords = [Vec2i::new(); 3];
        let mut world_coords = [Vec3f::new(); 3];
        for j in 0..3 {
            let v = model.data.position[face.0[j].0];
            screen_coords[j] = Vec2i::new_args(((v[0]+1.)*WIDTH as f32/2.) as i32, ((v[1]+1.)*HEIGHT as f32/2.) as i32);
            world_coords[j] = Vec3f::new_args(v[0], v[1], v[2]);
        }
        let mut n = (world_coords[2]-world_coords[0])^(world_coords[1]-world_coords[0]);
        n.normalize();
        let intensity = n*light_dir;
        if intensity>0.0 {
            triangle(screen_coords[0], screen_coords[1], screen_coords[2], &mut image, &TGAColor::new_rgba(intensity as u8, intensity as u8, intensity as u8, 255));
        }
    }

    image.flip_vertically();
    image.write_tga_file("/mnt/c/Users/shresr/Desktop/output.tga", false).unwrap();


}