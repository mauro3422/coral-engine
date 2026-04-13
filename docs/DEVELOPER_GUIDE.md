# 🪸 Coral Engine - Developer Guide

## Estado actual del proyecto
**Versión:** v0.4.0  
**Fecha:** 12 de abril de 2026  
**Lenguaje:** Rust  
**Motor gráfico:** wgpu (Vulkan/DX12/Metal)  
**UI:** egui  

---

## ¿Qué es Coral Engine?

Un motor gráfico especializado en **agua voxel animada**. A diferencia de los motores tradicionales que usan mallas planas con shaders de olas, Coral usa **cubos individuales (voxels)** que se animan con olas. Cada cubo es una unidad independiente.

### Visión del proyecto
Crear un océano voxel donde:
- Los bloques de agua se desarmen cuando un barco pasa
- Las tormentas generen olas que muevan bloques reales
- Cada voxel tenga su propia física y colisión
- Todo sea configurable y extensible

---

## Arquitectura actual

```
src/
├── ocean/                 # Sistema de agua (el core del motor)
│   ├── mod.rs             # Re-exports públicos
│   ├── config.rs          # OceanConfig + builder con validación
│   ├── world.rs           # OceanWorld + OceanDimensions
│   ├── block.rs           # WaterBlock (voxels individuales)
│   ├── render.rs          # WaterFace + block_types constants
│   └── bounds.rs          # ObjectBounds (estándar para wireframes)
│
├── core/                  # Infraestructura del motor
│   ├── camera.rs          # Cámara libre con constants
│   ├── input.rs           # InputState (teclado + mouse)
│   ├── config.rs          # RenderConfig
│   ├── scene.rs           # Scene, WorldObject, DebugObject
│   ├── cartesian.rs       # Axis3D, CoordinateSystem3D
│   └── ui_router.rs       # InputRouter para egui vs camera
│
├── render/                # GPU rendering
│   ├── state.rs           # RenderState (surface, device, queue)
│   ├── pipeline.rs        # VoxelPipeline + GridPipeline + shaders
│   ├── mesh.rs            # Mesh (cube, wireframe, grid, axes)
│   └── viewport_api.rs    # UI panels legacy
│
├── ui/                    # Editor UI (tipo Blender)
│   ├── mod.rs             # Re-exports
│   ├── editor.rs          # EditorState, Selection, Panels
│   └── panels.rs          # Outliner + Properties + Controls
│
├── game/                  # Game composition
│   └── state.rs           # Game struct
│
├── coordinator.rs         # Engine orchestrator (event loop, tick)
└── main.rs                # Entry point + winit event loop
```

---

## Conceptos clave

### Jerarquía de objetos
```
🌊 Ocean World        ← Gestiona todo el agua
   └── Water Blocks   ← Bloques de agua animados (1 por ahora)
        └── Voxels    ← Cubitos individuales
```

### Terminología de sliders
| Slider | Qué hace | Ejemplo |
|--------|----------|---------|
| **Voxel Size** | Tamaño de cada cubito | 0.5m = cubos de 50cm |
| **Block Size (m)** | Tamaño total del bloque de agua | 4.0m = bloque de 4 metros |
| **Water Layers** | Cuántas capas de agua hay | 4 = superficie de 4 cubos de alto |
| **Wave Height** | Amplitud de la animación de olas | 0.3m = olas suaves |
| **Wave Speed** | Velocidad de la animación | 1.0x = velocidad normal |

### Ejemplo concreto
Con Voxel Size = 0.5m, Block Size = 4.0m, Layers = 4:
- **Cubos por eje:** 4.0 ÷ 0.5 = 8
- **Voxels totales:** 8×8×4 = 256
- **Caras visibles:** ~512 (las expuestas al exterior)
- **Dimensiones del wireframe:** 4.0m × 2.0m × 4.0m

### Cómo funciona el Water Block
1. **Generación:** Se llena el bloque con voxels de agua (capas desde la base)
2. **Animación:** Cada frame se aplica un offset senoidal a la Y de cada voxel
   ```rust
   wave_offset = sin(time * 2.0 + x * 0.5 + z * 0.3) * wave_amplitude
   ```
3. **Face culling:** Solo se generan las caras expuestas (no las que tocan otro voxel de agua)
4. **GPU Instancing:** Cada cara visible es una instancia del mismo mesh de cubo
5. **Wireframe:** Se reconstruye automáticamente cuando cambia la config, alineado con el bloque

### Sincronización Wireframe ↔ Agua
- `OceanDimensions::bounds()` calcula el bounding box exacto del agua
- `rebuild_block_visualization()` usa ese bounds para crear el wireframe
- Se reconstruye en cada frame donde cambió un slider (`block_viz_dirty`)
- El wireframe **siempre** coincide con el agua

---

## UI del Editor

### Layout actual
```
┌────────────────────────────────┬─────────────┐
│                                │ 📋 Scene    │
│     3D VIEWPORT                │ 🌊 Water    │
│     (no bloqueado por UI)      ├─────────────┤
│                                │ 📐 Voxel    │
│                                │ 🧱 Block    │
│                                │ 🌊 Layers   │
│                                │ 📏 Wave H   │
│                                │ ⚡ Speed    │
│                                │ ▶️ Animation│
│                                │ 🎨 🌊📐🎯  │
└────────────────────────────────┴─────────────┘
                                🎮 Controls (window)
                                📊 Status bar (bottom)
```

### Panel derecho (Outliner + Properties)
- Lista de objetos en escena
- Click para seleccionar
- Sliders de configuración del agua
- Toggles de render layers

### Panel Controls (ventana flotante)
- Referencia rápida de teclas
- Colapsable, esquina inferior derecha

### Status Bar
- FPS, objetos, voxels activos
- Objeto seleccionado

---

## Controles de cámara
| Tecla | Acción |
|-------|--------|
| WASD | Movimiento horizontal |
| Space/Shift | Subir/Bajar |
| Q/E | Boost/Lentitud |
| Left Click | Capturar mouse para mirar |
| Scroll | Zoom (mueve cámara) |
| ESC | Salir |

---

## Estado de features

| Feature | Estado | Notas |
|---------|--------|-------|
| Agua voxel animada | ✅ Completo | 1 bloque, face culling, instancing |
| Wireframe sincronizado | ✅ Completo | Siempre alineado con el agua |
| UI editor | ✅ Completo | Sliders en vivo, propiedades |
| Cámara libre | ✅ Completo | WASD, mouse, zoom |
| GPU instancing | ✅ Completo | VoxelPipeline con shaders |
| Config persistente | ⏳ Parcial | No se guarda en archivo |
| Múltiples bloques | ❌ No empezado | Se puede extender |
| Terreno | ❌ No empezado | Módulo planeado |
| Colisiones | ❌ No empezado | Sistema planeado |
| Física de bloques | ❌ No empezado | Desarmado de bloques |
| Criaturas/entidades | ❌ No empezado | Planeado |
| Export/import | ❌ No empezado | Sin formato definido |

---

## Limitaciones conocidas

### Performance
- **Recalcular caras cada frame:** `collect_visible_faces()` itera todos los voxels cada frame. Se puede optimizar con dirty flags y caching.
- **Sin frustum culling de bloques:** Todos los bloques se procesan aunque no estén en vista.
- **Sin LOD:** La animación de olas se calcula igual para bloques cercanos y lejanos.

### UI
- **Sin persistencia:** Los cambios de configuración no se guardan.
- **Sin undo/redo:** No hay historial de cambios.
- **Sin multi-selección:** Solo se puede editar un objeto a la vez.

### Renderizado
- **Color de agua fijo:** `block_type: 7` hardcodeado en el shader.
- **Sin transparencias reales:** El agua usa alpha blend básico.
- **Sin iluminación dinámica:** Lighting simple Lambert.

---

## Próximos pasos (roadmap)

### Fase 1: Pulir el bloque actual
- [ ] Persistencia de configuración (archivo JSON/TOML)
- [ ] Undo/redo en sliders
- [ ] Mejorar iluminación del agua
- [ ] Color de agua configurable
- [ ] Optimizar `collect_visible_faces()` con caching

### Fase 2: Múltiples bloques
- [ ] Soporte para múltiples Water Blocks en escena
- [ ] Posicionar bloques individualmente
- [ ] Agregar bloques desde el editor
- [ ] Eliminar bloques

### Fase 3: Terreno
- [ ] Módulo `terrain/` siguiendo patrón ObjectBounds
- [ ] Generación procedural básica
- [ ] Arena bajo el agua
- [ ] Colisión agua-terreno

### Fase 4: Gameplay
- [ ] Personaje básico (cápsula)
- [ ] Movimiento sobre agua
- [ ] Nado dentro del agua
- [ ] Colisión con bloques

### Fase 5: Efectos
- [ ] Tormentas con olas amplificadas
- [ ] Bloques que se desarmen con impacto
- [ ] Splash particles
- [ ] Espuma en crestas de olas

---

## Comandos útiles

```bash
# Compilar y ejecutar
cargo run --release

# Build sin ejecutar
cargo build --release

# Limpiar caché (si los cambios no se reflejan)
cargo clean && cargo run --release

# Ver warnings
cargo clippy --release

# Push a GitHub
git add . && git commit -m "descripción" && git push
```

---

## Filosofía de diseño

1. **Voxel-first:** Todo son cubos. Las mallas solo cuando sea necesario.
2. **Configurable:** Todo parámetro es un slider en la UI.
3. **Sincronizado:** Wireframes, propiedades y estado siempre alineados.
4. **Extensible:** `ObjectBounds` es el estándar para cualquier objeto nuevo.
5. **Rust puro:** Sin atajos, sin engines externos.

---

## Créditos
- **Motor:** Coral Engine v0.4.0
- **Lenguaje:** Rust 2024
- **GPU:** wgpu 24
- **UI:** egui 0.31
- **Math:** cgmath 0.18
- **Window:** winit 0.30