mod geometry;
mod model;
mod tga;

use crate::{geometry::*, tga::*};
use model::Model;

const INPUT: &str = "obj/african_head.obj";
const OUTPUT: &str = "obj/output.tga";

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const DEPTH: i32 = 255;

pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> Matrix {
    let mut m = Matrix::identity(4);
    m[0][3] = (x + w) as f32 / 2.;
    m[1][3] = (y + h) as f32 / 2.;
    m[2][3] = DEPTH as f32 / 2.;

    m[0][0] = w as f32 / 2.;
    m[1][1] = h as f32 / 2.;
    m[2][2] = DEPTH as f32 / 2.;
    m
}

pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Matrix {
    let z = (eye - center).normalize().to_owned();
    let x = (up ^ z).normalize().to_owned();
    let y = (z ^ x).normalize().to_owned();
    let mut res = Matrix::identity(4);
    for i in 0..3 {
        res[0][i] = x[i];
        res[1][i] = y[i];
        res[2][i] = z[i];
        res[i][3] = -center[i];
    }
    res
}

pub fn triangle(
    mut t0: Vec3i,
    mut t1: Vec3i,
    mut t2: Vec3i,
    mut ity0: f32,
    mut ity1: f32,
    mut ity2: f32,
    image: &mut TGAImage,
    zbuffer: &mut [i32],
) {
    if t0.y == t1.y && t0.y == t2.y {}
    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
        swap(&mut ity0, &mut ity1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
        swap(&mut ity0, &mut ity2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
        swap(&mut ity1, &mut ity2);
    }
    println!("{:?}, {:?}, {:?}", t0, t1, t2);
    let total_height = t2.y - t0.y;
    for i in 0..total_height {
        let second_half = i > t1.y - t0.y || t1.y == t0.y;
        let segment_height = if second_half {
            t2.y - t1.y
        } else {
            t1.y - t0.y
        };
        let alpha = i as f32 / total_height as f32;
        let beta = (i - (if second_half { t1.y - t0.y } else { 0 })) as f32 / segment_height as f32;
        let mut A = t0 + Vec3f::into_float(t2 - t0) * alpha;
        let mut B = if second_half {
            t1 + Vec3f::into_float(t2 - t1) * beta
        } else {
            t0 + Vec3f::into_float(t1 - t0) * beta
        };
        let mut ityA = ity0 + (ity2 - ity0) * alpha;
        let mut ityB = if second_half {
            ity1 + (ity2 - ity1) * beta
        } else {
            ity0 + (ity1 - ity0) * beta
        };
        if A.x > B.x {
            swap(&mut A, &mut B);
            swap(&mut ityA, &mut ityB);
        }
        for j in A.x..=B.x {
            let phi = if B.x == A.x {
                1.
            } else {
                (j - A.x) as f32 / (B.x - A.x) as f32
            };
            let P = Vec3f::into_int(Vec3f::into_float(A) + Vec3f::into_float(B - A) * phi);
            let ityP = ityA + (ityB - ityA) * phi;
            let idx = (P.x + P.y * WIDTH) as usize;
            if P.x >= WIDTH || P.y >= HEIGHT || P.x < 0 || P.y < 0 {
                continue;
            }
            if zbuffer[idx] < P.z {
                zbuffer[idx] = P.z;
                image.set(P.x, P.y, &(TGAColor::new_rgba(255, 255, 255, 255) * ityP));
            }
        }
    }
}

fn main() {
    let model_wrapper = Model::new_args(INPUT);
    let model = model_wrapper.get_model();
    let mut zbuffer = [i32::MIN; (WIDTH * HEIGHT) as usize];
    let light_dir = Vec3f::new_args(1.0, -1.0, 1.0).normalize().to_owned();
    let eye = Vec3f::new_args(1.0, 1.0, 3.0);
    let center = Vec3f::new_args(0.0, 0.0, 0.0);

    {
        let ModelView = lookat(eye, center, Vec3f::new_args(0., 1., 0.));
        let mut Projection = Matrix::identity(4);
        let ViewPort = viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4);
        Projection[3][2] = -1. / (eye - center).norm();

        let z = ViewPort.clone() * Projection.clone() * ModelView.clone();

        let mut image = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.RGB as i32);
        for i in 0..model.data.objects[0].groups[0].polys.len() {
            let face = &model.data.objects[0].groups[0].polys[i];
            let mut screen_coords = [Vec3i::new(); 3];
            let mut world_coords = [Vec3f::new(); 3];
            let mut intensity = [0f32; 3];
            for j in 0..3 {
                let v = model.data.position[face.0[j].0];
                let v = Vec3f::new_args(v[0], v[1], v[2]);
                screen_coords[j] = Vec3f::into_int(Vec3f::from_matrix(
                    ViewPort.clone()
                        * Projection.clone()
                        * ModelView.clone()
                        * Matrix::from_vec3f(v),
                ));
                world_coords[j] = v;
                intensity[j] = model_wrapper.norm(i as i32, j as i32) * light_dir;
            }
            triangle(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                intensity[0],
                intensity[1],
                intensity[2],
                &mut image,
                &mut zbuffer,
            );
        }

        image.flip_vertically();
        image.write_tga_file(OUTPUT, false).unwrap();
    }
}
