// Core configuration - Render configuration with builder pattern
// Coral Engine - Render settings for viewport and display

/// Render configuration
#[derive(Clone, Debug)]
pub struct RenderConfig {
    pub show_water: bool,
    pub show_grid: bool,
    pub show_axes: bool,
    pub grid_size: f32,
    pub grid_divisions: u32,
    pub background_color: [f32; 4],
}

impl RenderConfig {
    pub fn default() -> Self {
        Self {
            show_water: true,
            show_grid: true,
            show_axes: true,
            grid_size: 100.0,
            grid_divisions: 20,
            background_color: [0.4, 0.6, 0.9, 1.0],
        }
    }

    pub fn builder() -> RenderConfigBuilder {
        RenderConfigBuilder::new()
    }
}

pub struct RenderConfigBuilder {
    config: RenderConfig,
}

impl RenderConfigBuilder {
    pub fn new() -> Self {
        Self { config: RenderConfig::default() }
    }

    pub fn show_water(mut self, show: bool) -> Self {
        self.config.show_water = show;
        self
    }

    pub fn show_grid(mut self, show: bool) -> Self {
        self.config.show_grid = show;
        self
    }

    pub fn show_axes(mut self, show: bool) -> Self {
        self.config.show_axes = show;
        self
    }

    pub fn grid_size(mut self, size: f32) -> Self {
        self.config.grid_size = size;
        self
    }

    pub fn grid_divisions(mut self, divisions: u32) -> Self {
        self.config.grid_divisions = divisions;
        self
    }

    pub fn background_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.config.background_color = [r, g, b, a];
        self
    }

    pub fn build(self) -> RenderConfig { self.config }
}

impl Default for RenderConfigBuilder {
    fn default() -> Self { Self::new() }
}
