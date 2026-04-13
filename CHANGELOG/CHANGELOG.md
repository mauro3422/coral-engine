# Changelog

All notable changes to Coral Engine will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.4.0] - 2026-04-12

### First Public Release - Coral Engine

#### Added
- **Ocean Water System** - Voxel-based water blocks with wave animation
  - Configurable voxel size (0.1-2.0m), block size (1-16m), water layers (1-4)
  - Real-time wave height and speed controls
  - GPU instancing for performant rendering
- **ObjectBounds API** - Standard bounding box system for automatic wireframe/collision sync
- **OceanDimensions** - Always-derived dimensions from actual config
- **Camera System** - Free camera with WASD, mouse look, zoom
  - Configurable speed, sensitivity, FOV, near/far planes
  - Custom defaults support via `Camera::with_config()`
- **Input System** - Keyboard and mouse handling with egui integration
  - WASD, Space/Shift, Q/E, ESC always functional
  - Click-and-drag mouse capture
  - Input routing with UI priority
- **UI Panels (egui)** - Live configuration panels
  - Ocean Config panel with sliders for all parameters
  - Statistics panel with FPS, voxel counts, camera position
  - Controls help panel
- **Block Visualization** - Orange wireframe that auto-syncs with water dimensions
- **Cartesian System** - Axis labels and gizmo colors

#### Architecture
- **Module Structure**:
  - `ocean/` - Water simulation (config, world, block, render, bounds)
  - `core/` - Engine infrastructure (camera, input, config, scene, ui_router)
  - `render/` - GPU rendering (state, pipeline, mesh, viewport_api)
  - `game/` - Game composition (state)
  - `coordinator.rs` - Engine orchestrator
- **Standardized APIs**:
  - `OceanConfig` with builder pattern and validation
  - `RenderConfig` with builder pattern
  - `ObjectBounds` as universal standard for any world object
  - Constants centralized in each module

#### Changed
- Renamed project from `motor-grafico` to `coral-engine`
- Reorganized from `systems/` graveyard to dedicated `ocean/` module
- Eliminated ~900 lines of dead code
- Reduced warnings from 67+ to 33 (mostly unused public APIs for future use)

#### Removed
- `systems/voxel.rs`, `systems/water.rs`, `systems/world.rs`, `systems/entities.rs`, `systems/physics.rs`
- `core/terrain.rs`
- `render/egui_overlay.rs`
- All duplicate code (axis gizmo, VoxelInstance types)

---

## [Unreleased]
