use obj::Obj;

use crate::{geometry::*, tga::*, model::Model};

pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> Matrix {
    let mut m = Matrix::identity(4);
    m[0][3] = (x + w) as f32 / 2.;
    m[1][3] = (y + h) as f32 / 2.;
    m[2][3] = 255. / 2.;

    m[0][0] = w as f32 / 2.;
    m[1][1] = h as f32 / 2.;
    m[2][2] = 255. / 2.;
    m
}

pub fn projection(coeff: f32) -> Matrix {
    let mut Projection = Matrix::identity(4);
    Projection[3][2] = coeff;
    Projection
}

pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Matrix {
    let z = (eye - center).normalize().to_owned();
    let x = cross(up, z).normalize().to_owned();
    let y = cross(z, x).normalize().to_owned();
    let mut res = Matrix::identity(4);
    for i in 0..3 {
        res[0][i] = x[i];
        res[1][i] = y[i];
        res[2][i] = z[i];
        res[i][3] = -center[i];
    }
    res
}

pub fn barycentric(A: Vec2f, B: Vec2f, C: Vec2f, P: Vec2f) -> Vec3f {
    let mut s = [Vec3f::new(); 2];
    for i in 0..2 {
        s[i][0] = C[i] - A[i];
        s[i][1] = B[i] - A[i];
        s[i][2] = A[i] - P[i];
    }
    let u = cross(s[0], s[1]);
    if u[2].abs()>1e-2 {
        return Vec3f::new_args(1.-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z)
    }
    Vec3f::new_args(-1., 1., 1.)
}


pub struct ViewBundle {
    pub ModelView: Matrix,
    pub ViewPort: Matrix,
    pub Projection: Matrix,
}

pub trait Shader {
    fn vertex(&mut self, iface: i32, nthvert: i32, model: &Obj, light_dir: Vec3f, view_bundle: &ViewBundle) -> Vec4f;
    fn fragment(&self, bar: Vec3f) -> (bool, TGAColor);
}

pub fn triangle(pts: &[Vec4f], shader: &impl Shader, image: &mut TGAImage, zbuffer: &mut TGAImage) {
    let mut bboxmin = Vec2f::new_args(f32::MAX, f32::MAX);
    let mut bboxmax = Vec2f::new_args(f32::MIN, f32::MIN);
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = bboxmin[j].min(pts[i][j]/pts[i][3]);
            bboxmax[j] = bboxmax[j].max(pts[i][j]/pts[i][3]);
        }
    }
    let mut P = Vec2i::new();
    for i in (bboxmin.x as i32)..=(bboxmax.x as i32) {
        P.x = i;
        for j in (bboxmin.y as i32)..=(bboxmax.y as i32) {
            P.y = j;
            let mut proj_P = Vec4f::new();
            proj_P[0] = P[0] as f32;
            proj_P[1] = P[1] as f32;
            let c = barycentric(proj(pts[0]/pts[0][3]), proj(pts[1]/pts[1][3]), proj(pts[2]/pts[2][3]), proj(proj_P));
            let z = pts[0][2]*c.x + pts[1][2]*c.y + pts[2][2]*c.z;
            let w = pts[0][3]*c.x + pts[1][3]*c.y + pts[2][3]*c.z;
            let frag_depth = std::cmp::max(0, std::cmp::min(255, (z/w+0.5) as i32)) as u8;
            if c.x<0. || c.y<0. || c.z<0. || zbuffer.get(P.x, P.y)[0] as i32>frag_depth.into() { continue; }
            let (discard, color) = shader.fragment(c);
            if !discard {
                zbuffer.set(P.x, P.y, &TGAColor::new_val(frag_depth as u32, 1));
                image.set(P.x, P.y, &color);
            }
        }
    }
}
