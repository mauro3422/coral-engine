# 🪸 Coral Engine

Motor gráfico voxel-based para renderizado de océanos en tiempo real. Hecho en **Rust** con **wgpu** y **egui**.

## 🌊 Visión

Un motor de juego donde el agua es el protagonista. Bloques de agua voxel con animación de olas, controles de cámara libres, y configuración en vivo. Diseñado para ser extensible a terreno, terreno submarino, y mundos oceánicos tipo Minecraft + Subnautica.

## ✨ Características

### Agua
- 🧊 **Bloques de agua voxel** configurables (no mallas planas)
- 🌊 **Animación de olas** con altura y velocidad ajustables en tiempo real
- 📐 **Tamaño de voxel configurable** (0.1m a 2.0m) - achicá cubos y aumentá la resolución
- 📏 **Bloques auto-escalables** - al cambiar el tamaño de voxel, la cantidad de cubos se recalcula automáticamente para mantener el tamaño físico del bloque
- 🎯 **Wireframe de sincronización automática** - el contorno naranja siempre coincide exactamente con el agua usando `ObjectBounds` y `OceanDimensions`
- 🎨 **GPU Instancing** - miles de cubos en un solo draw call

### Cámara
- 🎮 **WASD** - Movimiento libre
- ⬆️⬇️ **Space/Shift** - Subir/Bajar
- ⚡ **Q/E** - Velocidad rápida/lenta
- 🖱️ **Left Click + Drag** - Mirar alrededor
- 🔍 **Mouse Wheel** - Zoom (mueve la cámara, no solo FOV)
- ⎋ **ESC** - Salir

### UI en Vivo (egui)
| Panel | Controles |
|-------|-----------|
| 🌊 **Ocean Config** | Voxel Size, Block Size, Water Layers, Wave Height/Speed, Animation toggle |
| 📊 **Statistics** | FPS, Camera position, Active voxels, Visible faces, Block dimensions |
| 🎮 **Controls** | Ayuda de controles visibles |

### Arquitectura
- 📦 **Módulo `ocean/`** dedicado - Configuración, mundo, bloques, render, bounds
- 🔗 **API estandarizada** - `Config → Dimensions → ObjectBounds → Wireframe` para cualquier objeto futuro
- ✅ **Validación automática** - Todos los configs se validan y clamp al construir
- 🏗️ **Builder pattern** - `OceanConfig::builder()` y `RenderConfig::builder()`

## 🏗️ Arquitectura del Código

```
src/
├── ocean/                    # 🌊 Sistema de agua (API principal)
│   ├── mod.rs                # Re-exports públicos
│   ├── config.rs             # OceanConfig + OceanConfigBuilder (validado)
│   ├── world.rs              # OceanWorld + OceanDimensions
│   ├── block.rs              # WaterBlock (interno, no público)
│   ├── render.rs             # WaterFace + block_types constants
│   └── bounds.rs             # ObjectBounds (estándar universal)
│
├── core/                     # ⚙️ Infraestructura del motor
│   ├── camera.rs             # Cámara + constants (position, fov, speed...)
│   ├── input.rs              # InputState + constants
│   ├── config.rs             # RenderConfig + RenderConfigBuilder
│   ├── scene.rs              # Scene objects (grid, axes)
│   ├── cartesian.rs          # Axis labels, CoordinateSystem3D
│   └── ui_router.rs          # InputRouter, UiLayerRegistry
│
├── render/                   # 🎨 GPU rendering
│   ├── state.rs              # RenderState (surface, device, queue)
│   ├── pipeline.rs           # VoxelPipeline + GridPipeline + shaders
│   ├── mesh.rs               # Mesh (cube, wireframe, grid, axes)
│   └── viewport_api.rs       # UI panels (config, stats, controls)
│
├── game/                     # 🎮 Composición del juego
│   └── state.rs              # Game (ocean_world + viewport)
│
└── coordinator.rs            # 🔄 Engine orchestrator
    - Event handling (winit + egui)
    - Input routing (keyboard, mouse, camera)
    - Render loop (tick, update, render)
    - Config change detection
```

## 📐 API Estándar para Objetos

Cualquier objeto futuro (tierra, roca, edificios) sigue este patrón:

```rust
// 1. Tu Config
pub struct MyBlockConfig { ... }

// 2. Tus Dimensions (siempre derivadas de la config)
pub struct MyBlockDimensions {
    pub width: f32, pub height: f32, pub depth: f32, ...
}
impl MyBlockDimensions {
    pub fn from_config(config: &MyBlockConfig) -> Self { ... }
    pub fn bounds(&self) -> ObjectBounds { ... }
}

// 3. Wireframe automático
render_state.rebuild_block_visualization(my_block.dimensions());
```

**`ObjectBounds`** es el estándar universal para wireframes, colisiones y LOD de cualquier objeto en el mundo.

## 🚀 Compilar y Ejecutar

### Requisitos
- **Rust** (https://rustup.rs/)
- **Visual Studio Build Tools** (Windows) con "Desktop development with C++"
- **GPU** con soporte Vulkan, DirectX 12, o Metal

### Build
```bash
cargo build --release
cargo run --release
```

### Nota sobre caché
Si hacés cambios y no se reflejan:
```bash
cargo clean && cargo run --release
```

## 🎮 Controles Detallados

| Tecla | Acción | Detalle |
|-------|--------|---------|
| `W` | Avanzar | Velocidad base × boost/slow |
| `A` | Izquierda | Strafe left |
| `S` | Retroceder | Strafe back |
| `D` | Derecha | Strafe right |
| `Space` | Subir | Eje Y positivo |
| `Shift` | Bajar | Eje Y negativo |
| `Q` | Boost | Velocidad × 2.0 |
| `E` | Slow | Velocidad × 0.33 |
| `Left Click + Drag` | Mirar | Captura mouse |
| `Mouse Wheel` | Zoom | Mueve cámara en eje forward |
| `ESC` | Salir | Cierra la ventana |

## ⚙️ Configuración UI

### Voxel Size (0.1m - 2.0m)
Tamaño de cada cubo individual. Al achicarlo, la cantidad de cubos por bloque **aumenta automáticamente** para mantener el mismo tamaño físico del bloque.

| Voxel Size | Cubos/eje | Total cubos (1 bloque 4m) |
|------------|-----------|---------------------------|
| 0.10m | 40 | 64,000 |
| 0.25m | 16 | 4,096 |
| **0.50m** (default) | **8** | **512** |
| 1.00m | 4 | 64 |
| 2.00m | 2 | 8 |

### Block Physical Size (1.0m - 16.0m)
Tamaño físico total del bloque en metros. Cambia el tamaño real del agua en el mundo.

### Water Layers (1-4)
Cantidad de capas de agua. Solo se renderizan capas superficiales (no volumen 3D completo). **4 capas es el default** y simula bien el agua.

### Wave Height (0.0 - 2.0m)
Amplitud de la animación de olas en metros.

### Wave Speed (0.0 - 5.0x)
Velocidad de la animación.

## 📦 Estado del Proyecto

| Componente | Estado |
|------------|--------|
| Agua voxel | ✅ Funcional |
| Animación de olas | ✅ Funcional |
| Configuración en vivo | ✅ Funcional |
| Wireframe sincronizado | ✅ Funcional |
| Cámara libre | ✅ Funcional |
| UI panels | ✅ Funcional |
| GPU instancing | ✅ Funcional |
| Terreno | 🔲 Futuro |
| Física | 🔲 Futuro |
| Entidades/Mobs | 🔲 Futuro |
| Multiplayer | 🔲 Futuro |

## 📄 Licencia

MIT

## 🤝 Contribuir

```bash
git clone https://github.com/mauro3422/coral-engine.git
cd coral-engine
cargo run --release
```
