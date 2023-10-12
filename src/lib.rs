use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::util::DeviceExt;


// lib.rs
use winit::window::Window;

// lib.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

// lib.rs
impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}


// lib.rs
const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];


#[derive(Debug)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

#[derive(PartialEq, Eq)]
enum PipelineAlternatives {
    Default,
    Alternative1,
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    num_vertices: u32,
    bg_color: Color,
    pipeline_alternative: PipelineAlternatives,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let num_vertices = VERTICES.len() as u32;

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
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
                // we're building for the web we'll have to disable some.
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
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
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

        let bg_color = Color{
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        };

        let pipeline_alternative = PipelineAlternatives::Default;

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    Vertex::desc(),
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
                multisample: wgpu::MultisampleState {
                    count: 1, // 2.
                    mask: !0, // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
                multiview: None, // 5.
            });

        // new()
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );




        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            num_vertices,
            bg_color,
            pipeline_alternative,
            render_pipeline,
            vertex_buffer,
        }

    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    // impl State
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }


    fn input(&mut self, event: &WindowEvent) -> bool {
        match event{
            WindowEvent::CursorMoved { position, .. } => {
                let phys_pos = position.to_owned();
                let normalized_x = phys_pos.x / self.size.width as f64;
                let normalized_y = phys_pos.y / self.size.height as f64;

                self.bg_color.r = normalized_x;
                self.bg_color.g = normalized_y;
            },
            WindowEvent::KeyboardInput { 
                input:
                    KeyboardInput { 
                        state: ElementState::Pressed, 
                        virtual_keycode: Some(VirtualKeyCode::Space), 
                        ..
                    },
                ..
            } => {

                let device = &self.device;
                let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
                let render_pipeline_layout =
                    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: &[],
                        push_constant_ranges: &[],
                    });
                let render_pipeline: wgpu::RenderPipeline;

                if self.pipeline_alternative == PipelineAlternatives::Default {
                    self.pipeline_alternative = PipelineAlternatives::Alternative1;
                    render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("Render Pipeline"),
                        layout: Some(&render_pipeline_layout),
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: "vs_main", // 1.
                            buffers: &[
                                Vertex::desc(),
                            ], // 2.
                        },
                        fragment: Some(wgpu::FragmentState { // 3.
                            module: &shader,
                            entry_point: "fs_main",
                            targets: &[Some(wgpu::ColorTargetState { // 4.
                                format: self.config.format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                        }),
                        primitive: wgpu::PrimitiveState {
                            topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                            strip_index_format: None,
                            front_face: wgpu::FrontFace::Ccw, // 2.
                            cull_mode: Some(wgpu::Face::Back),
                            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                            polygon_mode: wgpu::PolygonMode::Fill,
                            // Requires Features::DEPTH_CLIP_CONTROL
                            unclipped_depth: false,
                            // Requires Features::CONSERVATIVE_RASTERIZATION
                            conservative: false,
                        },
                        depth_stencil: None, // 1.
                            multisample: wgpu::MultisampleState {
                                count: 1, // 2.
                                mask: !0, // 3.
                                alpha_to_coverage_enabled: false, // 4.
                            },
                            multiview: None, // 5.
                        });
                } else {
                    self.pipeline_alternative = PipelineAlternatives::Default;
                    render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("Render Pipeline"),
                        layout: Some(&render_pipeline_layout),
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: "vs_main", // 1.
                            buffers: &[
                                Vertex::desc(),
                            ], // 2.
                        },
                        fragment: Some(wgpu::FragmentState { // 3.
                            module: &shader,
                            entry_point: "fs2_main",
                            targets: &[Some(wgpu::ColorTargetState { // 4.
                                format: self.config.format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                        }),
                        primitive: wgpu::PrimitiveState {
                            topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                            strip_index_format: None,
                            front_face: wgpu::FrontFace::Ccw, // 2.
                            cull_mode: Some(wgpu::Face::Back),
                            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                            polygon_mode: wgpu::PolygonMode::Fill,
                            // Requires Features::DEPTH_CLIP_CONTROL
                            unclipped_depth: false,
                            // Requires Features::CONSERVATIVE_RASTERIZATION
                            conservative: false,
                        },
                        depth_stencil: None, // 1.
                            multisample: wgpu::MultisampleState {
                                count: 1, // 2.
                                mask: !0, // 3.
                                alpha_to_coverage_enabled: false, // 4.
                            },
                            multiview: None, // 5.
                        });
                }
                self.render_pipeline = render_pipeline;
            },
            _ => ()
        }
        false
    }

    fn update(&mut self) {
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
                            b: self.bg_color.b,
                            g: self.bg_color.g,
                            r: self.bg_color.r,
                            a: self.bg_color.a,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);

        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())

    }
}


pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| { 
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => 
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::CursorMoved { .. } => {
                        state.input(event);
                    },
                    WindowEvent::KeyboardInput { 
                        input:
                            KeyboardInput { 
                                state: ElementState::Pressed, 
                                virtual_keycode: Some(VirtualKeyCode::Space), 
                                ..
                            },
                        ..
                    } => {
                        state.input(event);
                    },
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    },
                    _ => {}
                },
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
            },
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            },
            _ => {}
        }
    });
}


