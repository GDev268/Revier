use wgpu::{util::DeviceExt, RequestAdapterOptions};
use winit::dpi::PhysicalSize;

use crate::window::Window;

pub struct Device {
    _device: wgpu::Device,
    surface: wgpu::Surface,
    pub queue: wgpu::Queue,
    surface_configuration: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter
}

impl Device {
    pub async fn new(window: &Window, backends: wgpu::Backends) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });

        let surface =
            unsafe { instance.create_surface(&window._window) }.expect("Failed to create surface!");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (_device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let capabilities = surface.get_capabilities(&adapter);

        let surface_configuration = wgpu::SurfaceConfiguration{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width: window.get_size().width,
            height: window.get_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![]
        };

        surface.configure(&_device, &surface_configuration);


        let depth_texture = _device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: window.width,
                height: window.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: Default::default(),
        });

        Self{
            _device,
            surface,
            queue,
            surface_configuration,
            adapter
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        return &self._device;
    }

    pub fn get_surface_format(&self) -> wgpu::TextureFormat {
        return self.surface_configuration.format;
    }
    
    pub fn get_surface(&self) -> &wgpu::Surface {
        return &self.surface;
    }

    pub fn resize(&mut self,new_size:PhysicalSize<u32>) {
        self.surface_configuration.width = new_size.width;
        self.surface_configuration.height = new_size.height;


        self.surface.configure(&self._device, &self.surface_configuration);
    }
}