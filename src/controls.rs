use raylib::prelude::*;

const VELOCIDAD_TECLADO: f32 = 1.5;
const SENSIBILIDAD_MOUSE: f32 = 0.01;

pub struct RotationControls {
    angle_x: f32,
    angle_y: f32,
}

impl RotationControls {
    pub fn new() -> Self {
        Self {
            angle_x: 0.0,
            angle_y: 0.0,
        }
    }

    pub fn update(&mut self, raylib: &RaylibHandle) {
        let tiempo_frame = raylib.get_frame_time();

        if raylib.is_key_down(KeyboardKey::KEY_LEFT) || raylib.is_key_down(KeyboardKey::KEY_A) {
            self.angle_y -= VELOCIDAD_TECLADO * tiempo_frame;
        }

        if raylib.is_key_down(KeyboardKey::KEY_RIGHT) || raylib.is_key_down(KeyboardKey::KEY_D) {
            self.angle_y += VELOCIDAD_TECLADO * tiempo_frame;
        }

        if raylib.is_key_down(KeyboardKey::KEY_UP) || raylib.is_key_down(KeyboardKey::KEY_W) {
            self.angle_x -= VELOCIDAD_TECLADO * tiempo_frame;
        }

        if raylib.is_key_down(KeyboardKey::KEY_DOWN) || raylib.is_key_down(KeyboardKey::KEY_S) {
            self.angle_x += VELOCIDAD_TECLADO * tiempo_frame;
        }

        if raylib.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let movimiento_mouse = raylib.get_mouse_delta();
            self.angle_y += movimiento_mouse.x * SENSIBILIDAD_MOUSE;
            self.angle_x += movimiento_mouse.y * SENSIBILIDAD_MOUSE;
        }
    }

    pub fn angles(&self) -> (f32, f32) {
        (self.angle_x, self.angle_y)
    }
}
