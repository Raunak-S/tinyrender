mod geometry;
mod model;
mod our_gl;
mod tga;

use crate::{geometry::*, our_gl::*, tga::*};
use model::*;
use num::pow;
use obj::Obj;

const INPUT: &str = "obj/african_head.obj";
const OUTPUT: &str = "obj/output.tga";
const ZBUFFER_OUTPUT: &str = "obj/zbuffer.tga";

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const DEPTH: f32 = 2000.;

struct Shader {
    uniform_M: Matrix,
    uniform_MIT: Matrix,
    uniform_Mshadow: Matrix,
    varying_uv: Matrix,
    varying_tri: Matrix,
}

impl Shader {
    pub fn new(M: Matrix, MIT: Matrix, MS: Matrix) -> Self {
        Self {
            uniform_M: M,
            uniform_MIT: MIT,
            uniform_Mshadow: MS,
            varying_uv: Matrix::new(Some(2), Some(3)),
            varying_tri: Matrix::new(Some(3), Some(3)),
        }
    }
}

impl IShader for Shader {
    fn vertex(&mut self, iface: i32, nthvert: i32, model: &Model, light_dir: Vec3f, view_bundle: &ViewBundle) -> Vec4f {
        self.varying_uv.set_col(nthvert, &vec![model.uv(iface, nthvert)[0], model.uv(iface, nthvert)[1]]);
        let gl_Vertex = view_bundle.ViewPort.clone()*view_bundle.Projection.clone()*view_bundle.ModelView.clone()*embed(&model.vert(iface, nthvert));
        self.varying_tri.set_col(nthvert, &proj_refactor((gl_Vertex/gl_Vertex[3]).into(), 3));
        gl_Vertex
    }

    fn fragment(&self, bar: Vec3f, model: &Model, shadowbuffer: &[f32], light_dir: &Vec3f) -> (bool, TGAColor) {
        let mut sb_p = self.uniform_Mshadow.clone()*embed(&Vec3f::from_vec(&(self.varying_tri.clone()*vec![bar[0], bar[1], bar[2]])));
        sb_p = sb_p/sb_p[3];
        let idx = sb_p[0] as i32 + WIDTH*sb_p[1] as i32;
        let shadow: f32 = 0.3+0.7*if shadowbuffer[idx as usize]<sb_p[2] { 1. } else { 0. };
        let uv = Vec2f::from_vec(&(self.varying_uv.clone()*vec![bar[0], bar[1], 0.]));
        let n = proj_refactor(self.uniform_MIT.clone()*Into::<Vec<f32>>::into(embed(&model.normal(&uv))), 3);
        let n = Vec3f { x: n[0], y: n[1], z: n[2] }.normalize().to_owned();
        let l = proj_refactor(self.uniform_M.clone()*Into::<Vec<f32>>::into(embed(light_dir)), 3);
        let l = Vec3f { x: l[0], y: l[1], z: l[2] }.normalize().to_owned();
        let r = (n*(n*l*2.) - l).normalize().to_owned();
        let spec = r.z.max(0.).powf(model.specular(&uv));
        let diff = 0f32.max(n*l);
        let c = model.diffuse(&uv);
        let mut color = TGAColor::new_rgba(255, 255, 255, 255);
        for i in 0..3 {
            color[i] = ((20. + c[i] as f32*shadow*(1.2*diff + 0.6*spec)) as u8).min(255);
        }
        (false, color)
    }
}

struct DepthShader {
    varying_tri: Matrix,
}

impl DepthShader {
    pub fn new() -> Self {
        Self { varying_tri: Matrix::new(Some(3), Some(3)) }
    }
}

impl IShader for DepthShader {
    fn vertex(&mut self, iface: i32, nthvert: i32, model: &Model, light_dir: Vec3f, view_bundle: &ViewBundle) -> Vec4f {
        let mut gl_Vertex = embed(&model.vert(iface, nthvert));
        gl_Vertex = view_bundle.ViewPort.clone()*view_bundle.Projection.clone()*view_bundle.ModelView.clone()*gl_Vertex;
        self.varying_tri.set_col(nthvert, &proj_refactor((gl_Vertex/gl_Vertex[3]).into(), 3));
        gl_Vertex
    }

    fn fragment(&self, bar: Vec3f, model: &Model, shadowbuffer: &[f32], light_dir: &Vec3f) -> (bool, TGAColor) {
        let p = Vec3f::from_vec(&(self.varying_tri.clone()*vec![bar[0], bar[1], bar[2]]));
        let color = TGAColor::new_rgba(255, 255, 255, 255)*(p.z/DEPTH);
        (false, color)
    }
}

fn main() {
    let mut zbuffer = [0f32; (WIDTH*HEIGHT) as usize];
    let mut shadowbuffer = [0f32; (WIDTH*HEIGHT) as usize];
    for i in 0..(WIDTH*HEIGHT) as usize {
        zbuffer[i] = f32::MIN;
        shadowbuffer[i] = f32::MIN;
    }

    let model_wrapper = Model::new_args(INPUT);
    let model = model_wrapper.get_model();
    let light_dir = Vec3f::new_args(1., 1., 0.).normalize().to_owned();
    
    let eye = Vec3f::new_args(1., 1., 4.);
    let center = Vec3f::new_args(0., 0., 0.);
    let up = Vec3f::new_args(0., 1., 0.);

    let mut ModelView: Matrix;
    let mut ViewPort: Matrix;
    let mut Projection: Matrix;
    let mut view_bundle: ViewBundle;
    
    ModelView = lookat(light_dir, center, up);
    ViewPort = viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4);
    Projection = projection(0.);
    view_bundle = ViewBundle { ModelView: ModelView.clone(), ViewPort: ViewPort.clone(), Projection: Projection.clone() };

    {
        let mut depth = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.RGB as i32);


        let mut depthshader = DepthShader::new();
        let mut screen_coords = [Vec4f::new(); 3];
        
        for i in 0..model.data.objects[0].groups[0].polys.len() {
            for j in 0..3 {
                screen_coords[j as usize] = depthshader.vertex(i as i32, j, &model_wrapper, light_dir, &view_bundle);
            }
            triangle(&screen_coords, &depthshader, &mut depth, &mut shadowbuffer, &model_wrapper,  None, &light_dir);
        }
        depth.flip_vertically();
        depth.write_tga_file("depth.tga", false).unwrap();
    }

    let M = ViewPort.clone()*Projection.clone()*ModelView.clone();

    ModelView = lookat(eye, center, up);
    ViewPort = viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4);
    Projection = projection(-1. / (eye - center).norm());
    view_bundle = ViewBundle { ModelView: ModelView.clone(), ViewPort: ViewPort.clone(), Projection: Projection.clone() };


    {
        let mut frame = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.RGB as i32);

        let mut shader = Shader::new(ModelView.clone(), (Projection.clone()*ModelView.clone()).invert_transpose(), M.clone()*(ViewPort.clone()*Projection.clone()*ModelView.clone()).invert());
        let mut screen_coords = [Vec4f::new(); 3];
        for i in 0..model.data.objects[0].groups[0].polys.len() {
            for j in 0..3 {
                screen_coords[j as usize] = shader.vertex(i as i32, j, &model_wrapper, light_dir, &view_bundle);
            }
            triangle(&screen_coords, &shader, &mut frame, &mut zbuffer, &model_wrapper, Some(&shadowbuffer), &light_dir);
        }
        frame.flip_vertically();
        frame.write_tga_file("framebuffer.tga", false).unwrap();
    }
}