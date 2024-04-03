use crate::{Input, WindowConfig, WindowImpl};
use anyhow::{anyhow, Result};
use std::{sync::Arc, time::Instant};

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, KeyCode, NamedKey, PhysicalKey},
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
    window::{Fullscreen, WindowBuilder},
};

mod crt;
mod fast_crt;
mod square;

use crt::CrtFilter;
use fast_crt::FastCrtFilter;
use square::SquareFilter;

pub struct Window {
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    palette_screen_mode: PaletteScreenMode,
    surface_config: wgpu::SurfaceConfiguration,
    filter: Box<dyn Filter>,
    event_loop: EventLoop<()>,
    window: Arc<winit::window::Window>,
    gamepads: [u8; 4],
    next_frame: Instant,
    is_fullscreen: bool,
    is_open: bool,
}

impl Window {
    pub fn new(window_config: WindowConfig) -> Result<Window> {
        async fn create(window_config: WindowConfig) -> Result<Window> {
            let event_loop = EventLoop::new()?;
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

            let window = Arc::new(window);

            let instance = wgpu::Instance::new(Default::default());
            let surface = instance.create_surface(window.clone())?;
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
        self.event_loop
            .set_control_flow(ControlFlow::WaitUntil(self.next_frame));
        while self.is_open {
            let timeout = self.next_frame.saturating_duration_since(Instant::now());
            let status = self.event_loop.pump_events(Some(timeout), |event, elwt| {
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
                            elwt.exit();
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            fn gamepad_button(input: &winit::event::KeyEvent) -> u8 {
                                match input.physical_key {
                                    PhysicalKey::Code(KeyCode::KeyZ) => 16,
                                    PhysicalKey::Code(KeyCode::KeyX) => 32,
                                    PhysicalKey::Code(KeyCode::KeyA) => 64,
                                    PhysicalKey::Code(KeyCode::KeyS) => 128,
                                    _ => match input.logical_key {
                                        Key::Named(NamedKey::ArrowUp) => 1,
                                        Key::Named(NamedKey::ArrowDown) => 2,
                                        Key::Named(NamedKey::ArrowLeft) => 4,
                                        Key::Named(NamedKey::ArrowRight) => 8,
                                        _ => 0,
                                    },
                                }
                            }
                            if event.state == winit::event::ElementState::Pressed {
                                match event.logical_key {
                                    Key::Named(NamedKey::Escape) => {
                                        elwt.exit();
                                    }
                                    Key::Character(ref c) => match c.as_str() {
                                        "f" => {
                                            let fullscreen = if self.window.fullscreen().is_some() {
                                                None
                                            } else {
                                                Some(Fullscreen::Borderless(None))
                                            };
                                            self.is_fullscreen = fullscreen.is_some();
                                            self.window.set_fullscreen(fullscreen);
                                        }
                                        "r" => reset = true,
                                        "1" => new_filter = Some(1),
                                        "2" => new_filter = Some(2),
                                        "3" => new_filter = Some(3),
                                        "4" => new_filter = Some(4),
                                        "5" => new_filter = Some(5),
                                        _ => (),
                                    },
                                    _ => (),
                                }

                                self.gamepads[0] |= gamepad_button(&event);
                            } else {
                                self.gamepads[0] &= !gamepad_button(&event);
                            }
                        }
                        _ => (),
                    },
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
            match status {
                PumpStatus::Exit(_) => self.is_open = false,
                _ => (),
            }

            if Instant::now() >= self.next_frame {
                break;
            }
        }
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
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
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
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
