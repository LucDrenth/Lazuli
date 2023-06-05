use std::path::Path;

use crate::graphics::{scene::Scene, material::Material, mesh_renderer, shader::{ShaderProgram, PATH_TEXTURED_FRAG, PATH_HELLO_TRANFORM_VERT}, Rectangle, Transform};

pub struct HelloTranslate {
    material1: Material,
    material2: Material,
    material3: Material,
    shape1: Rectangle,
    shape2: Rectangle,
    shape3: Rectangle,
    transform1: Transform,
    transform2: Transform,
    transform3: Transform,
    scale_direction1: f32,
    scale_direction2: f32,
    scale_direction3: f32,
}

impl HelloTranslate {
    pub fn new() -> Result<Self, String> {
        let program1 = ShaderProgram::new(PATH_HELLO_TRANFORM_VERT, PATH_TEXTURED_FRAG).unwrap();
        let mut material1 = Material::new(program1);
        material1.add_texture(&Path::new("./assets/images/pattern.png"));
        let shape1 = Rectangle::new_textured(&material1.shader_program);
        let mut transform1 = Transform::new();
        transform1.translate_x(-0.5);

        let program2 = ShaderProgram::new(PATH_HELLO_TRANFORM_VERT, PATH_TEXTURED_FRAG).unwrap();
        let mut material2 = Material::new(program2);
        material2.add_texture(&Path::new("./assets/images/pattern.png"));
        let shape2 = Rectangle::new_textured(&material2.shader_program);
        let mut transform2 = Transform::new();
        transform2.translate_y(0.5);

        let program3 = ShaderProgram::new(PATH_HELLO_TRANFORM_VERT, PATH_TEXTURED_FRAG).unwrap();
        let mut material3 = Material::new(program3);
        material3.add_texture(&Path::new("./assets/images/pattern.png"));
        let shape3 = Rectangle::new_textured(&material3.shader_program);
        let mut transform3 = Transform::new();
        transform3.translate_x(0.5);

        let result = Self { 
            material1,
            material2,
            material3,
            shape1,
            shape2,
            shape3,
            transform1,
            transform2,
            transform3,
            scale_direction1: 1.0,
            scale_direction2: 1.0,
            scale_direction3: -1.0,
        };

        Ok(result)
    }
}

impl Scene for HelloTranslate {
    fn update(&mut self) {
        // object 1
        self.transform1.scale(0.02 * self.scale_direction1);

        if self.transform1.scale.x > 1.5 {
            self.scale_direction1 = -1.0;
        } else if self.transform1.scale.x < 0.1 {
            self.scale_direction1 = 1.0;
        }

        self.transform1.rotate_z(0.02);

        self.material1.shader_program.set_uniform("transform", self.transform1.build());
        
        // object 2
        self.transform2.scale(0.02 * self.scale_direction2);

        if self.transform2.scale.x > 0.8 {
            self.scale_direction2 = -1.0;
        } else if self.transform2.scale.x < 0.1 {
            self.scale_direction2 = 1.0;
        }

        self.transform2.rotate_z(0.01);

        self.material2.shader_program.set_uniform("transform", self.transform2.build());

        // object 3
        self.transform3.scale(0.02 * self.scale_direction3);

        if self.transform3.scale.x > 1.5 {
            self.scale_direction3 = -1.0;
        } else if self.transform3.scale.x < 0.1 {
            self.scale_direction3 = 1.0;
        }

        self.transform3.rotate_z(0.02);

        self.material3.shader_program.set_uniform("transform", self.transform3.build());
    }

    unsafe fn draw(&self) {
        mesh_renderer::draw_rectangle(&self.shape1, &self.material1);
        mesh_renderer::draw_rectangle(&self.shape2, &self.material2);
        mesh_renderer::draw_rectangle(&self.shape3, &self.material3);
    }
}
