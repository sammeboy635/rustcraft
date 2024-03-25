use std::time::{Duration, Instant};

use texture::Texture;
use winit::event;
use winit::window::Window;
use wgpu::util::DeviceExt;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod texture;
mod camera;
mod camera3d;
mod uniformbuffer;
mod blockrender;
mod vertex;
mod globals;
mod cube;
mod atlas;

use crate::globals::*;
use crate::vertex::*;
use crate::camera::{CameraUniform, CameraUniformInitializer};
use crate::blockrender::*;


pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

 	let mut state = State::new(window).await;

	 event_loop.run(move |event, _, control_flow| {
		match event {
			Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
            } => state.camera.process_mouse_movement(delta.0 as f32, delta.1 as f32,1.0),
            
			Event::WindowEvent {
				ref event,
				window_id,
			} if window_id == state.window().id() => if !state.input(event, control_flow) { 
				match event {
					WindowEvent::Resized(physical_size) => {
						state.resize(*physical_size);
					}
					WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
						state.resize(**new_inner_size);
					}
					_ => {}
				}
			}
			Event::RedrawRequested(window_id) if window_id == state.window().id() => {
				state.update();
				match state.render() {
					Ok(_) => {}
					// Reconfigure the surface if lost
					Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
					// The system is out of memory, we should probably quit
					Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
					// All other errors (Outdated, Timeout) should be resolved by the next frame
					Err(e) => eprintln!("{:?}", e),
				}
			}
			Event::MainEventsCleared => {
				// RedrawRequested will only trigger once unless we manually
				// request it.
				state.window().request_redraw();
			}
			_ => {}
		}
	});
}
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

	//Renderer
	blockrender: BlockRender,
	depth_texture: Texture,
	//Camera
	camera: camera3d::Camera,

	//Timing
	last_render_time: Instant,
	dt: Duration,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,

}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window, so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

		let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

		let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

		// IMAGE
	
		// IMAGE END
		// Camera

		let camera = camera3d::Camera::default();

		let mut camera_uniform = camera::CameraUniform::new();
		camera_uniform.update_view_proj(&camera);
		let camera_wgpu = CameraUniformInitializer::new(&device, camera_uniform);

		// Camera End
		
		let modes = &surface_caps.present_modes;


		let mut blockrender = BlockRender::new(&device, &queue, &config, camera_wgpu);
		let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");

		Self {
            window,
            surface,
            device,
            queue,
            config,
            size,

			blockrender,
			depth_texture,
			camera,

			last_render_time: Instant::now(),
			dt: Duration::default(),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		self.camera.update_aspect(&self.window);
		self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");

		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	pub fn input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) -> bool {
		// self.camera.process_events(event);
		self.camera.keyboard_input(event);
		self.camera.mouse_input(event);
		return match event {
			WindowEvent::CursorMoved {position, ..} => {true}
			WindowEvent::CloseRequested => {*control_flow = ControlFlow::Exit; true},
			WindowEvent::KeyboardInput { input, .. } => {self.keyboard_input(input, control_flow)}
			_ => false
		};
	}

	pub fn keyboard_input(&mut self, input: &KeyboardInput, control_flow: &mut ControlFlow) -> bool {
		if input.state == ElementState::Pressed{
			return match input.virtual_keycode {
				Some(VirtualKeyCode::Escape) => {*control_flow = ControlFlow::Exit; true}
				Some(VirtualKeyCode::Space) => {true}
				_ => {println!("{:?}", input); false}
			}
		}
		false
	}

	fn update_time(&mut self){
		let now = instant::Instant::now();
		self.dt = now - self.last_render_time;
		self.last_render_time = now;
	}

	fn update(&mut self) {
		self.update_time();
		// self.camera_controller.update_camera(&mut self.camera);
		self.camera.process_keyboard_movement( self.dt.as_secs_f32());
		self.blockrender.camera_wgpu.uniform.update_view_proj(&self.camera);
		// self.camera_wgpu.uniform.update_view_proj(&self.camera);
		self.queue.write_buffer(&self.blockrender.camera_wgpu.buffer, 0, bytemuck::cast_slice(&[self.blockrender.camera_wgpu.uniform]));
	}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.1,
							g: 0.2,
							b: 0.3,
							a: 1.0,
						}),
						store: wgpu::StoreOp::Store,
					},
				})],
				depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
					view: &self.depth_texture.view,
					depth_ops: Some(wgpu::Operations {
						load: wgpu::LoadOp::Clear(1.0),
						store: wgpu::StoreOp::Store,
					}),
					stencil_ops: None,
				}),
				occlusion_query_set: None,
				timestamp_writes: None,
			});
			render_pass.set_pipeline(&self.blockrender.render_pipeline); // 2.
			render_pass.set_bind_group(0, &self.blockrender.diffuse_bind_group, &[]); 
			render_pass.set_bind_group(1, &self.blockrender.camera_wgpu.bind_group, &[]);
			render_pass.set_vertex_buffer(0, self.blockrender.vertex_buffer.slice(..));
			// render_pass.draw(0..(self.blockrender.vertex_buffer.size() / 3) as u32, 0..1)
			render_pass.set_index_buffer(self.blockrender.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
			render_pass.draw(0..36*2, 0..1);
			// render_pass.draw_indexed(indices, base_vertex, instances)
			// render_pass.draw_indexed(0..36 as u32, 0, 0..1); // 2.
			// render_pass.draw_indexed(36..36*2 as u32, 36, 0..1); // 2.

		}
	
		// submit will accept anything that implements IntoIter
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();
	
		Ok(())
    }
}
 