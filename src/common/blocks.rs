//! Block type constants and utilities
//! Centralized block type definitions

/// Block type IDs
pub mod block_types {
    /// Air/Void block
    pub const AIR: u8 = 0;
    /// Water block
    pub const WATER: u8 = 7;
    /// Sand block
    pub const SAND: u8 = 2;
    /// Dirt block
    pub const DIRT: u8 = 3;
    /// Stone block
    pub const STONE: u8 = 4;
    /// Grass block
    pub const GRASS: u8 = 5;
    /// Gravel block
    pub const GRAVEL: u8 = 6;
    /// Clay block
    pub const CLAY: u8 = 1;

    /// Get color for block type (RGB)
    pub fn color(id: u8) -> [f32; 3] {
        match id {
            AIR => [0.0, 0.0, 0.0],
            WATER => [0.1, 0.35, 0.7],
            SAND => [0.85, 0.75, 0.55],
            DIRT => [0.4, 0.25, 0.1],
            STONE => [0.5, 0.5, 0.5],
            GRASS => [0.2, 0.5, 0.15],
            GRAVEL => [0.4, 0.38, 0.35],
            CLAY => [0.6, 0.35, 0.25],
            _ => [0.5, 0.0, 0.5],
        }
    }

    /// Check if block is solid
    pub fn is_solid(id: u8) -> bool {
        id != AIR
    }

    /// Check if block is transparent
    pub fn is_transparent(id: u8) -> bool {
        id == AIR
    }

    /// Check if block is water-like
    pub fn is_water(id: u8) -> bool {
        id == WATER
    }
}

/// Validation helpers for configuration
pub mod validation {
    /// Clamp value to range
    pub fn clamp<T: Ord + Copy>(value: T, min: T, max: T) -> T {
        value.clamp(min, max)
    }

    /// Clamp and assign to field
    pub fn clamp_field<T: Ord + Copy>(field: &mut T, value: T, min: T, max: T) {
        *field = value.clamp(min, max);
    }

    /// Ensure positive value (minimum)
    pub fn positive<T: Ord + Copy + Default>(value: T, min: T) -> T {
        if value < min {
            min
        } else {
            value
        }
    }

    /// Ensure positive and assign
    pub fn positive_field<T: Ord + Copy + Default>(field: &mut T, value: T, min: T) {
        if value < min {
            *field = min
        } else {
            *field = value
        };
    }
}

/// Macro to generate a builder struct with fluent API
#[macro_export]
macro_rules! make_builder {
    (
        $(#[$meta:meta])*
        $builder:ident for $config:ident
    ) => {
        $(#[$meta])*
        pub struct $builder {
            config: $config,
        }

        impl $builder {
            pub fn new() -> Self {
                Self {
                    config: $config::default(),
                }
            }

            pub fn build(self) -> $config {
                self.config
            }
        }

        impl Default for $builder {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

/// Macro to generate getter/setter pairs with optional validation
#[macro_export]
macro_rules! make_accessors {
    (
        $struct:ident {
            $($field:ident: $ty:ty),* $(,)?
        }
    ) => {
        impl $struct {
            $(
                pub fn $field(&self) -> $ty {
                    self.$field
                }
            )*
        }
    };
}

/// Macro to generate validated setter
#[macro_export]
macro_rules! make_validator {
    (
        $struct:ident::$method:ident($value:ident : $ty:ty) -> $ret:ty
        where validate: $validation:expr
    ) => {
        impl $struct {
            pub fn $method(&mut self, $value: $ty) -> $ret {
                $validation
            }
        }
    };
}

/// Macro to derive Clone with explicit bounds
#[macro_export]
macro_rules! derive_clone {
    ($name:ident) => {
        impl Clone for $name {
            fn clone(&self) -> Self {
                Self { ..self }
            }
        }
    };
}
