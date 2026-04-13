# Sistema de Voxels - Especificacion

## Vision

El mundo está compuesto de voxels (cubos). Cada voxel es un dato independente que puede ser inspeccionado, modificado, removido o agregado en tiempo real.

Esta arquitectura permite:
- Breaking/placing de bloques en tiempo real
- Mundo procedural infinito
- Multiplayer facil (solo sincronizar cambios de voxels)
- Colisiones naturales por bloque

## Estructura de Datos

### VoxelData

```rust
struct VoxelData {
    block_type: u8,    // 0 = aire, 1+ = tipos de bloque
    light: u8,        // Nivel de luz (0-15)
    flags: u8,         // Flags especiales
}
```

- Max 255 tipos de bloque (suficiente para inventario completo)
- Luz para día/noche y blockes que emitan luz
- Flags: transparente, liquido, solido, interactuable

### BlockType

```rust
struct BlockType {
    name: &'static str,
    texture_id: u16,
    transparent: bool,
    liquid: bool,
    walkable: bool,
    light_emission: u8,
}
```

Registros de tipos de bloque predefinidos:
- AIRE = 0 (void/no hay bloque)
- PIEDRA = 1
- TIERRA = 2
- ARENA = 3
- MADERA = 4
- HOJAS = 5
- AGUA = 6 (liquido, transparente)
- VIDRIO = 7 (transparente)
- LADRILLO = 8
-obsidiana = 9

Extension por mods/config:
- 100+ custom blocks

### Chunk

```rust
const CHUNK_SIZE_X: u32 = 16;
const CHUNK_SIZE_Y: u32 = 128;
const CHUNK_SIZE_Z: u32 = 16;

struct Chunk {
    voxels: [VoxelData; CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z],
    position: IVec3,  // Posicion delchunk en coordenadas de mundo
    modified: bool,
}
```

- Fixed: 16x16x128 = 32,768 voxels
- Esto es ~32KB por chunk (sin comprimir)
- Chunks se cargan/des cargan desde/hacia disco
- Solo cargar chunks visibles + 1 de distancia

### Mundo (World)

```rust
struct VoxelWorld {
    chunks: HashMap<IVec3, Chunk>,
    chunks_dirty: Vec<IVec3>,
}
```

- HashMap para acceso O(1) por chunk position
- Chunks modificados se guardan en disco
- Solo manteneren memoria chunks cercanos

### Coordendas

- Mundo usa coordenadas globales (i32, i32, i32)
- Chunk coords = voxel_coords / 16
- Voxel local = voxel_coords % 16
- Y positivo = arriba

## Rendering

### GPU Instancing

Para renderizar miles de cubes:

1. **Un mesh de cube** (12 triangles, 8 vertices)
2. **Instance buffer** contiene:
   - Vec3 posicion (instanceposition * 1.0)
   - u8 tipo de bloque
   - u8 luz

3. **Un draw call** dibuja todas las instances visibles

```wgsl
// Vertex shader
struct Instance {
    position: vec3<f32>,
    block_type: u8,
}
@group(0) @binding(0) var<uniform> instances: array<Instance>;
```

### Culling

Solo dibujar cubes visibles:
- No dibujar aire (block_type == 0)
- No dibujar cubes tapados por 6 lados (optimizacion opcional)
- Frustum culling por chunk

###chunk Render

1. Para cada chunk visible:
   - Recorrer voxels
   - Si no aire → agregar a instance buffer
2. Update instance buffer en GPU
3. Un draw call por chunk (o agrupar chunks)

### Luz (Futuro)

- Propagation de luz simple: 4-adjacent
- Blockes con light_emission > 0 iluminan
- Ambient occlusion opcional

## Colisiones

Cada voxel tiene hitbox natural:

```
AABB {
    min: (chunk_x, chunk_y, chunk_z),
    max: (chunk_x + 1, chunk_y + 1, chunk_z + 1),
}
```

-Collision check: verificar voxel en posicion del player
- Player puede caminar si el voxel debajo es solido

### Fisica del Player

```rust
// Player usa AABB
struct PlayerPhysics {
    position: Vec3<f32>,  // Centro del player
    velocity: Vec3<f32>,
    on_ground: bool,
}

// Collision:
// 1. Buscar voxel donde está el player
// 2. Si no solido → gravity
// 3. Si solido → stop
// 4. Verificar 4 esquinas del AABB
```

## Generacion Procedural

### Superficies

```rust
fn generate_height(x: i32, z: i32) -> i32 {
    // Simplex noise para altura base
    let base = noise(x * 0.01, z * 0.01) * 32 + 32;
    
    // Biome blend opcional
    let biome = get_biome(x, z);
    
    match biome {
        DESERT => base + 4,
        FOREST => base,
        OCEAN => base - 8,
    }
}
```

### Relleno

```rust
fn generate_voxel(x: i32, y: i32, z: i32) -> VoxelData {
    let surface = generate_height(x, z);
    
    if y < surface - 4 {
        PIEDRA
    } else if y < surface {
        TIERRA
    } else if y < surface + 1 {
        // Capa superior
        match get_bieme(x, z) {
            DESERT => ARENA,
            FOREST => GRASS,
            OCEAN => ARENA,
        }
    } else if y < WATER_LEVEL && get_biome(x, z) == OCEAN {
        AGUA
    } else {
        AIRE
    }
}
```

## Serializacion

### Guardar chunks

```rust
fn save_chunk(chunk: &Chunk) -> Vec<u8> {
    // Guardar solo voxels modificados
    let mut data = Vec::new();
    for (i, voxel) in chunk.voxels.iter().enumerate() {
        if voxel.block_type != 0 {
            data.push(i as u16);
            data.push(voxel.block_type);
        }
    }
    compress(data)
}
```

### Cargar chunks

- Cargar chunk desde disco si existe
- Si no, generar con noise
- Marcar como "generated" para seeds

## Extensiones Futuras

### Multiplayer

```rust
// Solo enviar cambios
struct VoxelChange {
    position: IVec3,
    block_type: u8,
}
// Enviar este evento, no estado completo
```

### Mods

```rust
// Agregar tipos de bloque custom
fn register_block(name: &str, block_type: BlockType) {
    // Registrartexture, sonido,etc
}
```

### Animaciones

- Agua: cambiar Y de voxels Periodicamente
- Lava: glow animation
- Fire: particles encima

### Animales/Mobs

```rust
// Mob = conjunto animado de voxels
struct Mob {
    voxels: Array<AABB>,  // Cuerpo de cubos
    animation: Animation,
}
```

## Resumen de Vision

| Componente | Status | Prioridad |
|----------|--------|----------|
| VoxelData + BlockTypes | Implementar | Alta |
| Chunk + World | Implementar | Alta |
| Instanced Rendering | Implementar | Alta |
| Collision | Implementar | Alta |
| Generacion procedural | Implementar | Media |
| Guardar/Cargar | Despues | Baja |
| Multiplayer | Despues | Baja |

Philosophy: **Voxel-first**, meshes solo cuando sea necesario.

Los personajes, edificios, items - todo comienza como cubos.
Solo usamos mesh tradicional cuando el rendimiento lo justifique.