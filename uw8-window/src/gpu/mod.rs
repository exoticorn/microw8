use crate::{Input, WindowConfig, WindowImpl};
use anyhow::{anyhow, Result};
use std::time::Instant;

use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};

use winit::platform::run_return::EventLoopExtRunReturn;

mod crt;
mod fast_crt;
mod square;

use crt::CrtFilter;
use fast_crt::FastCrtFilter;
use square::SquareFilter;

pub struct Window {
    _instance: wgpu::Instance,
    surface: wgpu::Surface,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    palette_screen_mode: PaletteScreenMode,
    surface_config: wgpu::SurfaceConfiguration,
    filter: Box<dyn Filter>,
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    gamepads: [u8; 4],
    next_frame: Instant,
    is_fullscreen: bool,
    is_open: bool,
}

impl Window {
    pub fn new(window_config: WindowConfig) -> Result<Window> {
        async fn create(window_config: WindowConfig) -> Result<Window> {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_inner_size(PhysicalSize::new(
                    (320. * window_config.scale).round() as u32,
                    (240. * window_config.scale).round() as u32,
                ))
                .with_min_inner_size(PhysicalSize::new(320u32, 240))
                .with_title("MicroW8")
                .with_fullscreen(if window_config.fullscreen {
                    Some(Fullscreen::Borderless(None))
                } else {
                    None
                })
                .build(&event_loop)?;

            window.set_cursor_visible(false);

            let instance = wgpu::Instance::new(Default::default());
            let surface = unsafe { instance.create_surface(&window) }?;
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::LowPower,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .ok_or_else(|| anyhow!("Request adapter failed"))?;

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default(), None)
                .await?;

            let palette_screen_mode = PaletteScreenMode::new(&device);

            let surface_config = wgpu::SurfaceConfiguration {
                present_mode: wgpu::PresentMode::AutoNoVsync,
                ..surface
                    .get_default_config(
                        &adapter,
                        window.inner_size().width,
                        window.inner_size().height,
                    )
                    .expect("Surface incompatible with adapter")
            };

            let filter: Box<dyn Filter> = create_filter(
                &device,
                &palette_screen_mode.screen_view,
                window.inner_size(),
                surface_config.format,
                window_config.filter,
            );

            surface.configure(&device, &surface_config);

            Ok(Window {
                event_loop,
                window,
                _instance: instance,
                surface,
                _adapter: adapter,
                device,
                queue,
                palette_screen_mode,
                surface_config,
                filter,
                gamepads: [0; 4],
                next_frame: Instant::now(),
                is_fullscreen: window_config.fullscreen,
                is_open: true,
            })
        }

        pollster::block_on(create(window_config))
    }
}

impl WindowImpl for Window {
    fn begin_frame(&mut self) -> Input {
        let mut reset = false;
        self.event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(self.next_frame);
            let mut new_filter = None;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new_size) => {
                        self.surface_config.width = new_size.width;
                        self.surface_config.height = new_size.height;
                        self.surface.configure(&self.device, &self.surface_config);
                        self.filter.resize(&self.queue, new_size);
                    }
                    WindowEvent::CloseRequested => {
                        self.is_open = false;
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        fn gamepad_button(input: &winit::event::KeyboardInput) -> u8 {
                            match input.scancode {
                                44 => 16,
                                45 => 32,
                                30 => 64,
                                31 => 128,
                                _ => match input.virtual_keycode {
                                    Some(VirtualKeyCode::Up) => 1,
                                    Some(VirtualKeyCode::Down) => 2,
                                    Some(VirtualKeyCode::Left) => 4,
                                    Some(VirtualKeyCode::Right) => 8,
                                    _ => 0,
                                },
                            }
                        }
                        if input.state == winit::event::ElementState::Pressed {
                            match input.virtual_keycode {
                                Some(VirtualKeyCode::Escape) => {
                                    self.is_open = false;
                                    *control_flow = ControlFlow::Exit;
                                }
                                Some(VirtualKeyCode::F) => {
                                    let fullscreen = if self.window.fullscreen().is_some() {
                                        None
                                    } else {
                                        Some(Fullscreen::Borderless(None))
                                    };
                                    self.is_fullscreen = fullscreen.is_some();
                                    self.window.set_fullscreen(fullscreen);
                                }
                                Some(VirtualKeyCode::R) => reset = true,
                                Some(VirtualKeyCode::Key1) => new_filter = Some(1),
                                Some(VirtualKeyCode::Key2) => new_filter = Some(2),
                                Some(VirtualKeyCode::Key3) => new_filter = Some(3),
                                Some(VirtualKeyCode::Key4) => new_filter = Some(4),
                                Some(VirtualKeyCode::Key5) => new_filter = Some(5),
                                _ => (),
                            }

                            self.gamepads[0] |= gamepad_button(&input);
                        } else {
                            self.gamepads[0] &= !gamepad_button(&input);
                        }
                    }
                    _ => (),
                },
                Event::RedrawEventsCleared => {
                    if Instant::now() >= self.next_frame
                        // workaround needed on Wayland until the next winit release
                        && self.window.fullscreen().is_some() == self.is_fullscreen
                    {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => (),
            }
            if let Some(new_filter) = new_filter {
                self.filter = create_filter(
                    &self.device,
                    &self.palette_screen_mode.screen_view,
                    self.window.inner_size(),
                    self.surface_config.format,
                    new_filter,
                );
            }
        });
        Input {
            gamepads: self.gamepads,
            reset,
        }
    }

    fn end_frame(&mut self, framebuffer: &[u8], palette: &[u8], next_frame: Instant) {
        self.next_frame = next_frame;
        self.palette_screen_mode
            .write_framebuffer(&self.queue, framebuffer);
        self.palette_screen_mode.write_palette(&self.queue, palette);

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.palette_screen_mode.resolve_screen(&mut encoder);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.filter.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn is_open(&self) -> bool {
        self.is_open
    }
}

fn create_filter(
    device: &wgpu::Device,
    screen_texture: &wgpu::TextureView,
    window_size: PhysicalSize<u32>,
    surface_format: wgpu::TextureFormat,
    filter: u32,
) -> Box<dyn Filter> {
    match filter {
        1 => Box::new(SquareFilter::new(
            device,
            screen_texture,
            window_size,
            surface_format,
        )),
        2 => Box::new(FastCrtFilter::new(
            device,
            screen_texture,
            window_size,
            surface_format,
            false,
        )),
        3 => Box::new(CrtFilter::new(
            device,
            screen_texture,
            window_size,
            surface_format,
        )),
        4 => Box::new(FastCrtFilter::new(
            device,
            screen_texture,
            window_size,
            surface_format,
            true,
        )),
        _ => Box::new(AutoCrtFilter::new(
            device,
            screen_texture,
            window_size,
            surface_format,
        )),
    }
}

trait Filter {
    fn resize(&mut self, queue: &wgpu::Queue, new_size: PhysicalSize<u32>);
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

struct AutoCrtFilter {
    small: CrtFilter,
    large: FastCrtFilter,
    resolution: PhysicalSize<u32>,
}

impl AutoCrtFilter {
    fn new(
        device: &wgpu::Device,
        screen: &wgpu::TextureView,
        resolution: PhysicalSize<u32>,
        surface_format: wgpu::TextureFormat,
    ) -> AutoCrtFilter {
        let small = CrtFilter::new(device, screen, resolution, surface_format);
        let large = FastCrtFilter::new(device, screen, resolution, surface_format, true);
        AutoCrtFilter {
            small,
            large,
            resolution,
        }
    }
}

impl Filter for AutoCrtFilter {
    fn resize(&mut self, queue: &wgpu::Queue, new_size: PhysicalSize<u32>) {
        self.small.resize(queue, new_size);
        self.large.resize(queue, new_size);
        self.resolution = new_size;
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.resolution.width < 960 || self.resolution.height < 720 {
            self.small.render(render_pass);
        } else {
            self.large.render(render_pass);
        }
    }
}

struct PaletteScreenMode {
    framebuffer: wgpu::Texture,
    palette: wgpu::Texture,
    screen_view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl PaletteScreenMode {
    fn new(device: &wgpu::Device) -> PaletteScreenMode {
        let framebuffer_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        });

        let palette_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D1,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        });

        let screen_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });

        let framebuffer_texture_view =
            framebuffer_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let palette_texture_view =
            palette_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let screen_texture_view =
            screen_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let palette_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Uint,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D1,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                ],
                label: None,
            });

        let palette_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &palette_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&framebuffer_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&palette_texture_view),
                },
            ],
            label: None,
        });

        let palette_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("palette.wgsl").into()),
        });

        let palette_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&palette_bind_group_layout],
                push_constant_ranges: &[],
            });

        let palette_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&palette_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &palette_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &palette_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        });

        PaletteScreenMode {
            framebuffer: framebuffer_texture,
            palette: palette_texture,
            screen_view: screen_texture_view,
            bind_group: palette_bind_group,
            pipeline: palette_pipeline,
        }
    }

    fn write_framebuffer(&self, queue: &wgpu::Queue, pixels: &[u8]) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.framebuffer,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &bytemuck::cast_slice(pixels),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(320),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
        );
    }

    fn write_palette(&self, queue: &wgpu::Queue, palette: &[u8]) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.palette,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &bytemuck::cast_slice(palette),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(256 * 4),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
    }

    fn resolve_screen(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.screen_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
