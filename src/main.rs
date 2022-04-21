mod geometry;
mod model;
mod our_gl;
mod tga;

use crate::{geometry::*, our_gl::*, tga::*};
use model::*;
use time::Instant;

const INPUT: &str = "/home/raunaks/Projects/tinyrender/obj/FinalBaseMesh.obj";

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

const light_dir: Vec3f = Vec3f {
    x: 1.,
    y: 1.,
    z: 1.,
};
const eye: Vec3f = Vec3f {
    x: 1.,
    y: 1.,
    z: 3.,
};
const center: Vec3f = Vec3f {
    x: 0.,
    y: 0.,
    z: 0.,
};
const up: Vec3f = Vec3f {
    x: 0.,
    y: 1.,
    z: 0.,
};

struct Shader<'a> {
    model: &'a Model,
    uniform_l: Vec3f,
    varying_uv: Matrix,
    varying_nrm: Matrix,
    view_tri: Matrix,
}

impl<'a> Shader<'a> {
    pub fn new(m: &'a Model, view_bundle: &ViewBundle) -> Self {
        Self {
            model: m,
            uniform_l: *Vec3f::from_vec(&proj_refactor(
                Into::<Vec<f32>>::into(view_bundle.ModelView.clone() * embed(&light_dir, Some(0.))),
                3,
            ))
            .normalize(),
            varying_uv: Matrix::new(Some(2), Some(3)),
            varying_nrm: Matrix::new(Some(3), Some(3)),
            view_tri: Matrix::new(Some(3), Some(3)),
        }
    }
}

impl<'a> IShader for Shader<'a> {
    fn vertex(&mut self, iface: i32, nthvert: i32, view_bundle: &ViewBundle) -> Vec4f {
        self.varying_uv.set_col(
            nthvert,
            &vec![
                self.model.uv(iface, nthvert)[0],
                self.model.uv(iface, nthvert)[1],
            ],
        );
        self.varying_nrm.set_col(
            nthvert,
            &proj_refactor(
                Into::<Vec<f32>>::into(
                    (view_bundle.ModelView.clone()).invert_transpose()
                        * embed(&self.model.norm(iface, nthvert), Some(0.)),
                ),
                3,
            ),
        );
        let mut gl_Position =
            view_bundle.ModelView.clone() * embed(&self.model.vert(iface, nthvert), None);
        self.view_tri.set_col(
            nthvert,
            &proj_refactor(Into::<Vec<f32>>::into(gl_Position), 3),
        );
        gl_Position = view_bundle.Projection.clone() * gl_Position;
        return gl_Position;
    }

    fn fragment(&self, bar: Vec3f) -> (bool, TGAColor) {
        let bn = Vec3f::from_vec(&(self.varying_nrm.clone() * vec![bar[0], bar[1], bar[2]]))
            .normalize()
            .to_owned();
        let uv = Vec2f::new_args(
            (self.varying_uv.clone() * vec![bar[0], bar[1], bar[2]])[0],
            (self.varying_uv.clone() * vec![bar[0], bar[1], bar[2]])[1],
        );
        // TODO: this matrix definition might be wrong
        let mut AI = Matrix::new(Some(3), Some(3));
        let col0 = Vec3f::from_vec(&self.view_tri.col(0));
        let col1 = Vec3f::from_vec(&self.view_tri.col(1));
        let col2 = Vec3f::from_vec(&self.view_tri.col(2));
        AI[0] = [(col1 - col0)[0], (col1 - col0)[1], (col1 - col0)[2], 0.];
        AI[1] = [(col2 - col0)[0], (col2 - col0)[1], (col2 - col0)[2], 0.];
        AI[2] = [bn[0], bn[1], bn[2], 0.];
        AI = AI.invert();

        let mut i = Vec3f::from_vec(
            &(AI.clone()
                * vec![
                    self.varying_uv[0][1] - self.varying_uv[0][0],
                    self.varying_uv[0][2] - self.varying_uv[0][0],
                    0.,
                ]),
        );
        let mut j = Vec3f::from_vec(
            &(AI.clone()
                * vec![
                    self.varying_uv[1][1] - self.varying_uv[1][0],
                    self.varying_uv[1][2] - self.varying_uv[1][0],
                    0.,
                ]),
        );
        let mut B = Matrix::new(Some(3), Some(3));
        B[0] = [
            i.normalize().to_owned()[0],
            i.normalize().to_owned()[1],
            i.normalize().to_owned()[2],
            0.,
        ];
        B[1] = [
            j.normalize().to_owned()[0],
            j.normalize().to_owned()[1],
            j.normalize().to_owned()[2],
            0.,
        ];
        B[2] = [bn[0], bn[1], bn[2],0.];
        B = B.transpose();

        let n = Vec3f::from_vec(
            &(B * vec![
                self.model.normal(&uv)[0],
                self.model.normal(&uv)[1],
                self.model.normal(&uv)[2],
            ]),
        )
        .normalize()
        .to_owned();
        let diff = 0f32.max(n * self.uniform_l);
        let r = (n * (n * self.uniform_l) * 2. - self.uniform_l)
            .normalize()
            .to_owned();
        let spec = (-r.z)
            .max(0.)
            .powf(5. + Shader::sample2D(self.model.specular(), &uv)[0] as f32);

        let c = Shader::sample2D(self.model.diffuse(), &uv);
        let mut gl_FragColor = TGAColor::new_rgba(255, 255, 255, 0);
        for i in 0..3 {
            gl_FragColor[i] = (10. + c[i] as f32 * (diff + spec).min(255.)) as u8;
        }
        (false, gl_FragColor)
    }
}

fn main() {
    let mut framebuffer = TGAImage::new_dimensions(WIDTH, HEIGHT, TGAFormat.RGB as i32);
    let ModelView = lookat(eye, center, up);
    let ViewPort = viewport(WIDTH / 8, HEIGHT / 8, WIDTH * 3 / 4, HEIGHT * 3 / 4);
    let Projection = projection((eye - center).norm());
    let view_bundle = ViewBundle {
        ModelView: ModelView.clone(),
        ViewPort: ViewPort.clone(),
        Projection: Projection.clone(),
    };
    let mut zbuffer = vec![f32::MAX; (WIDTH * HEIGHT) as usize];

    let model_wrapper = Model::new_args(INPUT);

    let mut shader = Shader::new(&model_wrapper, &view_bundle);
    println!("Rendering {} triangles", model_wrapper.nfaces());
    let now = Instant::now();
    for i in 0..model_wrapper.nfaces() {
        let mut clip_vert = [Vec4f::new(); 3];
        for j in 0..3 {
            clip_vert[j as usize] = shader.vertex(i as i32, j, &view_bundle);
        }
        //if i == 30 { panic!() }
        triangle(
            &clip_vert,
            &shader,
            &mut framebuffer,
            &mut zbuffer,
            &view_bundle,
        );
    }
    println!("Finished in {}", now.elapsed());
    framebuffer.flip_vertically();
    framebuffer
        .write_tga_file(
            "/home/raunaks/Projects/tinyrender/obj/framebuffer.tga",
            false,
        )
        .unwrap();
}
