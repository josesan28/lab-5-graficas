use raylib::prelude::*;

pub struct Framebuffer {
    width: i32,
    height: i32,
    color_buffer: Vec<Vector3>,
    background_color: Vector3,
    texture: Option<Texture2D>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![Vector3::zero(); (width * height) as usize],
            background_color: Vector3::zero(),
            texture: None,
        }
    }

    pub fn set_background_color(&mut self, color: Vector3) {
        self.background_color = color;
    }

    pub fn init_texture(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        let image = Image::gen_image_color(self.width, self.height, Color::BLACK);
        self.texture = Some(
            window
                .load_texture_from_image(thread, &image)
                .expect("No se pudo crear la textura del framebuffer"),
        );
    }

    pub fn clear(&mut self) {
        self.color_buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x: i32, y: i32, color: Vector3) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }

        let index = (y * self.width + x) as usize;
        self.color_buffer[index] = color;
    }

    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        let pixels: Vec<u8> = self
            .color_buffer
            .iter()
            .flat_map(|color| {
                [
                    Self::color_component(color.x),
                    Self::color_component(color.y),
                    Self::color_component(color.z),
                    255,
                ]
            })
            .collect();

        let texture = self
            .texture
            .as_mut()
            .expect("La textura del framebuffer no ha sido inicializada");
        texture
            .update_texture(&pixels)
            .expect("No se pudo actualizar la textura del framebuffer");

        let mut drawing = window.begin_drawing(thread);
        drawing.clear_background(Color::BLACK);
        drawing.draw_texture(texture, 0, 0, Color::WHITE);
    }

    fn color_component(value: f32) -> u8 {
        (value.clamp(0.0, 1.0) * 255.0) as u8
    }
}
