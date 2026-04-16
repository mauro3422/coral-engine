//! Core configuration - Render configuration with builder pattern
//! Coral Engine - Render settings for viewport and display

use std::default::Default;

/// Render configuration
#[derive(Clone, Debug, Default)]
pub struct RenderConfig {
    pub show_water: bool,
    pub show_grid: bool,
    pub show_axes: bool,
    pub grid_size: f32,
    pub grid_divisions: u32,
    pub background_color: [f32; 4],
}

impl RenderConfig {
    /// Create default render config
    pub fn new() -> Self {
        Self::default()
    }

    // Getters
    pub fn show_water(&self) -> bool {
        self.show_water
    }
    pub fn show_grid(&self) -> bool {
        self.show_grid
    }
    pub fn show_axes(&self) -> bool {
        self.show_axes
    }
    pub fn grid_size(&self) -> f32 {
        self.grid_size
    }
    pub fn grid_divisions(&self) -> u32 {
        self.grid_divisions
    }
    pub fn background_color(&self) -> [f32; 4] {
        self.background_color
    }

    // Setters
    pub fn set_show_water(&mut self, show: bool) {
        self.show_water = show;
    }
    pub fn set_show_grid(&mut self, show: bool) {
        self.show_grid = show;
    }
    pub fn set_show_axes(&mut self, show: bool) {
        self.show_axes = show;
    }
    pub fn set_grid_size(&mut self, size: f32) {
        self.grid_size = size.max(1.0);
    }
    pub fn set_grid_divisions(&mut self, div: u32) {
        self.grid_divisions = div.clamp(1, 100);
    }
    pub fn set_background_color(&mut self, color: [f32; 4]) {
        self.background_color = color;
    }

    /// Create builder for fluent configuration
    pub fn builder() -> RenderConfigBuilder {
        RenderConfigBuilder::new()
    }
}

/// Builder for RenderConfig
pub struct RenderConfigBuilder {
    config: RenderConfig,
}

impl RenderConfigBuilder {
    /// Create new builder with defaults
    pub fn new() -> Self {
        Self {
            config: RenderConfig::default(),
        }
    }

    /// Set show_water flag
    pub fn show_water(mut self, show: bool) -> Self {
        self.config.show_water = show;
        self
    }

    /// Set show_grid flag
    pub fn show_grid(mut self, show: bool) -> Self {
        self.config.show_grid = show;
        self
    }

    /// Set show_axes flag
    pub fn show_axes(mut self, show: bool) -> Self {
        self.config.show_axes = show;
        self
    }

    /// Set grid size
    pub fn grid_size(mut self, size: f32) -> Self {
        self.config.grid_size = size.max(1.0);
        self
    }

    /// Set grid divisions
    pub fn grid_divisions(mut self, divisions: u32) -> Self {
        self.config.grid_divisions = divisions.clamp(1, 100);
        self
    }

    /// Set background color (RGBA)
    pub fn background_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.config.background_color = [r, g, b, a];
        self
    }

    /// Build config
    pub fn build(self) -> RenderConfig {
        self.config
    }
}

impl Default for RenderConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
