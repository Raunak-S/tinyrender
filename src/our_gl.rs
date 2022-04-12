use crate::{geometry::*, tga::*};

pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> Matrix {
    let mut m = Matrix::new(Some(4), Some(4));
    m[0] = vec![w as f32 / 2., 0., 0., x as f32 + w as f32 / 2.];
    m[1] = vec![0., h as f32 / 2., 0., y as f32 + h as f32 / 2.];
    m[2] = vec![0., 0., 1., 0.];
    m[3] = vec![0., 0., 0., 1.];
    m
}

pub fn projection(f: f32) -> Matrix {
    let mut Projection = Matrix::new(Some(4), Some(4));
    Projection[0] = vec![1., 0., 0., 0.];
    Projection[1] = vec![0., -1., 0., 0.];
    Projection[2] = vec![0., 0., 1., 0.];
    Projection[3] = vec![0., 0., -1. / f, 0.];
    Projection
}

pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Matrix {
    let z = (center - eye).normalize().to_owned();
    let x = cross(up, z).normalize().to_owned();
    let y = cross(z, x).normalize().to_owned();
    let mut Minv = Matrix::new(Some(4), Some(4));
    Minv[0] = vec![x.x, x.y, x.z, 0.];
    Minv[1] = vec![y.x, y.y, y.z, 0.];
    Minv[2] = vec![z.x, z.y, z.z, 0.];
    Minv[3] = vec![0., 0., 0., 1.];
    let mut Tr = Matrix::new(Some(4), Some(4));
    Tr[0] = vec![1., 0., 0., -eye.x];
    Tr[1] = vec![0., 1., 0., -eye.y];
    Tr[2] = vec![0., 0., 1., -eye.z];
    Tr[3] = vec![0., 0., 0., 1.];
    Minv * Tr
}

pub fn barycentric(tri: &[Vec2f; 3], P: &Vec2f) -> Vec3f {
    let mut ABC = Matrix::new(Some(3), Some(3));
    ABC[0] = embed_refactor::<3>(&vec![tri[0][0], tri[0][1]], None);
    ABC[1] = embed_refactor::<3>(&vec![tri[1][0], tri[1][1]], None);
    ABC[2] = embed_refactor::<3>(&vec![tri[2][0], tri[2][1]], None);
    if ABC.det() < 1e-3 {
        return Vec3f::new_args(-1., 1., 1.);
    }
    Vec3f::from_vec(&(ABC.invert_transpose() * embed_refactor::<3>(&vec![P[0], P[1]], None)))
}

pub struct ViewBundle {
    pub ModelView: Matrix,
    pub ViewPort: Matrix,
    pub Projection: Matrix,
}

pub trait IShader {
    fn sample2D(img: &TGAImage, uvf: &Vec2f) -> TGAColor {
        img.get(
            (uvf[0] * img.get_width() as f32) as i32,
            (uvf[1] * img.get_height() as f32) as i32,
        )
    }
    fn vertex(&mut self, iface: i32, nthvert: i32, view_bundle: &ViewBundle) -> Vec4f;
    fn fragment(&self, bar: Vec3f) -> (bool, TGAColor);
}

pub fn triangle(
    clip_verts: &[Vec4f],
    shader: &impl IShader,
    image: &mut TGAImage,
    zbuffer: &mut Vec<f32>,
    view_bundle: &ViewBundle,
) {
    let Viewport = view_bundle.ViewPort.clone();

    let pts = [
        Viewport.clone() * clip_verts[0],
        Viewport.clone() * clip_verts[1],
        Viewport.clone() * clip_verts[2],
    ];
    let pts2 = [
        proj(pts[0] / pts[0][3]),
        proj(pts[1] / pts[1][3]),
        proj(pts[2] / pts[2][3]),
    ];

    let mut bboxmin = Vec2f::new_args(f32::MAX, f32::MAX);
    let mut bboxmax = Vec2f::new_args(f32::MIN, f32::MIN);
    let clamp = Vec2f::new_args(
        (image.get_width() - 1) as f32,
        (image.get_height() - 1) as f32,
    );
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = (bboxmin[j].min(pts2[i][j])).max(0.);
            bboxmax[j] = (bboxmax[j].max(pts2[i][j])).min(clamp[j]);
        }
    }

    for x in (bboxmin.x as i32)..=(bboxmax.x as i32) {
        for y in (bboxmin.y as i32)..=(bboxmax.y as i32) {
            let bc_screen = barycentric(&pts2, &Vec2f::new_args(x as f32, y as f32));
            let mut bc_clip = Vec3f::new_args(
                bc_screen.x / pts[0][3],
                bc_screen.y / pts[1][3],
                bc_screen.z / pts[2][3],
            );
            bc_clip = bc_clip / (bc_clip.x + bc_clip.y + bc_clip.z);
            let frag_depth =
                Vec3f::new_args(clip_verts[0][2], clip_verts[1][2], clip_verts[2][2]) * bc_clip;
            if bc_screen.x < 0.
                || bc_screen.y < 0.
                || bc_screen.z < 0.
                || frag_depth > zbuffer[(x + y * image.get_width()) as usize]
            {
                continue;
            }
            let (discard, color) = shader.fragment(bc_clip);
            if discard {
                continue;
            }
            zbuffer[(x + y * image.get_width()) as usize] = frag_depth;
            image.set(x, y, &color);
        }
    }
}
