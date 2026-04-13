# 🪸 Coral Engine

Motor gráfico voxel-based para renderizado de océanos en Rust.

## Visión

Un mundo 3D hecho de cubos donde el agua es el protagonista:
- 🌊 Océano voxel con animación de olas en tiempo real
- 📐 Bloques configurables con wireframe de sincronización automática
- 🎮 Controles de cámara completos (WASD, mouse, zoom)
- ⚙️ UI con egui para configuración en vivo
- 🚀 GPU instancing para rendimiento

## Arquitectura

```
src/
├── ocean/          # Sistema de agua (API principal)
│   ├── config.rs   # OceanConfig + builder
│   ├── world.rs    # OceanWorld + OceanDimensions
│   ├── block.rs    # WaterBlock (interno)
│   ├── render.rs   # WaterFace + block_types
│   └── bounds.rs   # ObjectBounds (estándar)
├── core/           # Infraestructura del motor
│   ├── camera.rs   # Cámara con constants
│   ├── input.rs    # InputState
│   ├── config.rs   # RenderConfig
│   ├── scene.rs    # Scene objects
│   ├── cartesian.rs # Sistema de coordenadas
│   └── ui_router.rs # Input routing para egui
├── render/         # GPU rendering
│   ├── state.rs    # RenderState
│   ├── pipeline.rs # VoxelPipeline + GridPipeline
│   ├── mesh.rs     # Mesh definitions
│   └── viewport_api.rs # UI panels
├── game/           # Game composition
│   └── state.rs    # Game struct
└── coordinator.rs  # Engine orchestrator
```

## Controles

| Tecla | Acción |
|-------|--------|
| WASD | Mover cámara |
| Space/Shift | Subir/Bajar |
| Q/E | Velocidad rápida/lenta |
| Left Click + Drag | Mirar alrededor |
| Mouse Wheel | Zoom |
| ESC | Salir |

## Compilar

```bash
cargo build --release
cargo run --release
```

## Configuración UI

Los sliders del panel izquierdo permiten ajustar en vivo:
- **Voxel Size**: Tamaño de cada cubo (0.1-2.0m)
- **Block Size**: Tamaño físico del bloque (1-16m)
- **Water Layers**: Capas de agua (1-4)
- **Wave Height**: Amplitud de olas (0-2m)
- **Wave Speed**: Velocidad de animación (0-5x)
