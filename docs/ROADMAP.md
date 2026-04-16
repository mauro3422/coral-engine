# Coral Engine - Development Roadmap

## Project Overview

**Coral Engine** es un motor gráfico voxel-based en Rust (wgpu + egui) para renderizado de océanos en tiempo real.

**Versión actual**: 0.5.1
**Target**: 0.6.0 (Physics + Entities)

---

## 📋 Completed Features (v0.4.0)

- [x] Voxel water blocks con animación de olas
- [x] GPU instancing para renderizado eficiente
- [x] Cámara libre WASD + mouse look
- [x] Configuración en tiempo real (voxel size, block size, waves)
- [x] UI panels (config, stats, controls)
- [x] Scene object system con selección
- [x] Wireframe synchronization con ObjectBounds
- [x] SQLite persistence básica

---

## 🎯 Phase 1: Input System (v0.5.0a)

**Timeline**: 2 semanas (Semanas 1-2)

### 1.1 Core Infrastructure
- [x] `KeyModifiers` struct (ctrl, shift, alt, logo)
- [x] `KeyCombo` struct con Display impl
- [x] `InputContext` enum expansion (ViewMode, ObjectMode, EditMode, PaintMode)
- [x] `ActionCategory` enum
- [x] `InputAction` enum expansion completo

### 1.2 Action System
- [x] `ActionMap` con soporte para KeyCombo
- [x] `ContextActionMap` con Blender-style defaults
- [x] `ActionState` con just_pressed/just_released tracking
- [x] `InputManager` CREATED (not yet integrated)

### 1.3 Integration
- [x] **Update `coordinator.rs` para usar InputManager**
- [x] Migrar todas las queries de input
- [x] Testing de funcionalidad existente

### 1.4 Persistence
- [x] SQLite schema para keymaps
- [x] Save/load keymaps
- [x] Keymap editor UI panel

---

## 🎯 Phase 2: Viewport/Layout System (v0.5.0b)

**Timeline**: 4 semanas (Semanas 3-6)

### 2.1 Layout Core
- [x] `LayoutNode` enum (Panel, Split, Floating, Viewport)
- [x] `LayoutState` con Blender-style default
- [x] Panel rendering básico (integración activa)

### 2.2 Resize System
- [x] Splitter drag detection
- [x] Size calculation desde ratios
- [x] Min/max constraints

### 2.3 Animation System
- [x] `PanelAnimation` struct
- [x] Easing functions (ease-out cubic)
- [x] Animation queue y update

### 2.4 Multi-Viewport
- [x] `ViewportManager`
- [x] Viewport creation/deletion
- [x] Camera sync entre viewports

### 2.5 Panel Registry
- [x] `PanelContentTrait`
- [x] Built-in panel implementations
- [x] Panel visibility system (toggle with hotkeys)

**Hotkeys** (en desarrollo):
- `K` = Keymap Editor
- `L` = Layout Editor
- `O` = Outliner panel
- `P` = Properties panel
- `1` = Stats panel
- `2` = Controls panel
- `WASD` = Move camera
- `E` = Interact/select
- `Q` = Descender
- `Shift` = Speed boost

---

## 🎯 Phase 3: Features (v0.6.0+)

### Terrain System
- [x] Voxel terrain blocks (`src/terrain/`)
- [x] Terrain generation (noise-based)
- [x] Terrain rendering
- [ ] Terrain integration to ocean

### Physics (Basic)
- [x] AABB collision detection (`src/physics/`)
- [x] Simple gravity
- [x] Entity physics sync
- [x] Raycast support

### Entities/Mobs
- [x] Entity system framework (`src/entity/`)
- [x] Basic NPC placeholders
- [x] Entity spawning
- [ ] Entity rendering integration

### Multiplayer (Future)
- [ ] Network abstraction
- [ ] State synchronization

---

## 📊 Architecture Overview

```
src/
├── coordinator.rs        # Main orchestrator
├── core/
│   ├── camera.rs        # Free camera
│   ├── input.rs        # Input handling (reemplazado por InputManager)
│   ├── input_actions.rs # Action system (v0.5)
│   ├── scene.rs        # Scene objects
│   ├── ui_router.rs    # UI/World input router
│   ├── database.rs     # SQLite persistence
│   └── cartesian.rs    # Coordinate system
├── render/
│   ├── state.rs        # Render orchestration
│   ├── pipeline.rs    # GPU pipelines
│   ├── mesh.rs         # Mesh builders
│   └── viewport_api.rs # Viewport UI (reemplazado por Layout)
├── ocean/
│   ├── config.rs      # OceanConfig
│   ├── world.rs        # OceanWorld
│   ├── bounds.rs       # ObjectBounds
│   └── render.rs       # Water rendering
├── game/
│   └── state.rs        # Game state
└── ui/
    ├── panels.rs       # UI panels (reemplazado por Layout)
    ├── editor.rs       # Editor state
    ├── block_panel.rs # Block browser
    └── scene_panel.rs # Scene management
```

---

## 🔧 Key Technical Decisions

### 1. Input System
- **Blender-style** como default keymap
- Context-based action mapping
- Key combos (Ctrl+S, etc.)
- Persistencia SQLite

### 2. Layout System
- **Tree-based** layout structure
- Egui-native rendering
- Animated resizes
- SQLite persistence

### 3. Rendering
- **GPU Instancing** para voxels
- Separate pipelines (voxel, grid, axes)
- Depth-first render pass
- Egui overlay pass

### 4. Data
- SQLite para persistence
- In-memory scene snapshots
- ObjectBounds como estándar

---

## 📝 Documentation

| File | Description |
|------|-------------|
| `docs/INPUT_SYSTEM_SPEC.md` | Input system v0.5 spec detallada |
| `docs/LAYOUT_VIEWPORT_SPEC.md` | Layout/viewport system spec |
| `docs/INPUT_AND_UI.md` | UI/Input router actual |
| `docs/ARCHITECTURE.md` | Arquitectura general |
| `docs/PROJECT_OVERVIEW.md` | Estado del proyecto |

---

## 🚀 Development Workflow

1. **Plan** → Spec documentada en `docs/`
2. **Implement** → Code en `src/`
3. **Test** → Manual + cargo check
4. **Iterate** → Basado en feedback

---

## ⏱️ Timeline Summary

| Phase | Timeline | Focus |
|-------|----------|-------|
| 1 | Semanas 1-2 | Input System |
| 2 | Semanas 3-6 | Layout/Viewport |
| 3 | Semanas 7-10 | Terrain + Physics |
| 4 | Semanas 11+ | Entities + Polish |

**Total target: v0.6.0 en ~3 meses**

---

## 🤝 Contributing

```bash
# Dev workflow
cargo check    # Verificar compilación
cargo run      # Ejecutar
cargo build    # Build release
```

**Estado**: Prototipo funcional, en desarrollo activo
**Meta**: Motor de juego completo con editor estilo Blender