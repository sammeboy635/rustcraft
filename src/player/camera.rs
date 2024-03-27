use cgmath::{Deg, InnerSpace, Matrix4, Point3, Vector3, Transform};
use winit::dpi::PhysicalPosition;
use crate::uniform::CameraUniform;
use winit::{
    event::*,
    window::WindowBuilder,
};

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
	pub movement_direction: Vector3<f32>,
	pub movement_speed: f32,
	pub inverted_mouse: bool,
    pub camera_uniform: CameraUniform,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Point3::new(0.0, 0.0, 5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 500.0,
            movement_direction: Vector3::new(0.0,0.0,0.0),
			movement_speed: 5.0,
			inverted_mouse: true,
            camera_uniform: CameraUniform::new(),
        }
    }
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
		movement_direction: Vector3<f32>,
		movement_speed: f32,
		inverted_mouse: bool,
        camera_uniform: CameraUniform,
    ) -> Self {
        Camera {
            position,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
			movement_direction,
			movement_speed,
			inverted_mouse,
            camera_uniform,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.position, self.target, self.up);
        let proj = cgmath::perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }
    pub fn update_view_projection(&mut self){
        self.camera_uniform.view_proj = self.build_view_projection_matrix().into();
    }

    pub fn update_aspect(&mut self, window: &winit::window::Window) {
        let size = window.inner_size();
        self.aspect = size.width as f32 / size.height as f32;
    }

	pub fn process_mouse_movement(
		&mut self,
		mouse_dx: f32,
		mouse_dy: f32,
		sensitivity: f32,
	) {
		let mouse_dx = if self.inverted_mouse { -mouse_dx } else { mouse_dx };
		let mouse_dy = if self.inverted_mouse { -mouse_dy } else { mouse_dy };

		let mouse_dx = mouse_dx * sensitivity;
		let mouse_dy = mouse_dy * sensitivity;
	
		let yaw = Deg(mouse_dx);
		let pitch = Deg(mouse_dy);
	
		let direction = self.target - self.position;
		let distance = direction.magnitude();
	
		let pitch_axis = direction.cross(self.up).normalize();
		let yaw_axis = self.up;
	
		let rotation_pitch = Matrix4::from_axis_angle(pitch_axis, pitch);
		let rotation_yaw = Matrix4::from_axis_angle(yaw_axis, yaw);
		let rotation = rotation_yaw * rotation_pitch;
	
		let rotated_direction = rotation.transform_vector(direction).normalize();
	
		self.target = self.position + rotated_direction * distance;
	}


	pub fn process_keyboard_movement(
		&mut self,
		dt: f32,
	) {
		// Calculate the forward direction based on the camera's orientation
		let forward = (self.target - self.position).normalize();
		let right = forward.cross(self.up).normalize();
		let up = right.cross(forward);

	
		// Transform the movement direction based on the camera's orientation
		let movement = self.movement_direction.x * right +
					   self.movement_direction.y * up +
					   self.movement_direction.z * forward;
	
		let velocity = movement * self.movement_speed * dt;
		self.position += velocity;
		self.target += velocity;
	}
	
	pub fn keyboard_input(&mut self, event: &WindowEvent) -> bool {
		match event {
			WindowEvent::KeyboardInput {
				input:
					KeyboardInput {
						state,
						virtual_keycode: Some(keycode),
						..
					},
				..
			} => {
				let is_pressed = *state == ElementState::Pressed;
				let amount = if is_pressed { 1.0 } else { 0.0 };
				match keycode {
					VirtualKeyCode::W | VirtualKeyCode::Up => {
						self.movement_direction.z = amount;
						true
					}
					VirtualKeyCode::S | VirtualKeyCode::Down => {
						self.movement_direction.z = -amount;
						true
					}
					VirtualKeyCode::A | VirtualKeyCode::Left => {
						self.movement_direction.x = -amount;
						true
					}
					VirtualKeyCode::D | VirtualKeyCode::Right => {
						self.movement_direction.x = amount;
						true
					}
					VirtualKeyCode::Space => {
						self.movement_direction.y = amount;
						true
					}
					VirtualKeyCode::LShift => {
						self.movement_direction.y = -amount;
						true
					}
					_ => false,
				}
			}
			_ => false,
		}
	}

	pub fn mouse_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                // self.is_mouse_left_pressed = *state == ElementState::Pressed;
                true
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Right,
                ..
            } => {
                // self.is_mouse_right_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }
}

