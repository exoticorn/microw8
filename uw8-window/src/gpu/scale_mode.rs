use winit::dpi::PhysicalSize;

#[derive(Debug, Copy, Clone)]
pub enum ScaleMode {
    Fit,
    Fill,
}

impl Default for ScaleMode {
    fn default() -> ScaleMode {
        ScaleMode::Fit
    }
}

impl ScaleMode {
    pub fn texture_scale_from_resolution(&self, res: PhysicalSize<u32>) -> [f32; 4] {
        let scale = match self {
            ScaleMode::Fit => ((res.width as f32) / 160.0).min((res.height as f32) / 120.0),
            ScaleMode::Fill => ((res.width as f32) / 160.0).max((res.height as f32) / 120.0),
        };
        [
            scale / res.width as f32,
            scale / res.height as f32,
            2.0 / scale,
            0.0,
        ]
    }
}
