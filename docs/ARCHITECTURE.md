# Arquitectura

## Capas principales

### 1. `core`

Contiene la infraestructura base del motor:
- `camera`: cámara libre y control de movimiento
- `cartesian`: definición del sistema cartesiano
- `input`: estado de entrada
- `scene`: snapshot renderizable de la escena
- `terrain`: estructura de malla por grilla con alturas
- `ui_router`: router de input entre UI y mundo

### 2. `systems`

Contiene sistemas de gameplay y simulación:
- `world`: generación de terreno, isla y objetos de prueba
- `water`: deformación simple del agua
- `entities`: entidades lógicas de escena
- `physics`: cuerpos físicos y sincronización con entidades

### 3. `render`

Contiene el pipeline de GPU y la traducción de escena a draw calls:
- `pipeline`: creación de pipelines y shaders embebidos
- `mesh`: construcción de meshes de CPU
- `state`: orquestación del render por frame
- `egui_overlay`: paneles de debug y viewport

### 4. `game`

Une los sistemas anteriores para construir la escena final por frame.

## Flujo de frame

1. `coordinator` recibe eventos de ventana y mouse.
2. `InputRouter` decide si la UI reclama el puntero o el teclado.
3. `Game` actualiza agua, física y entidades.
4. `Game::build_scene()` arma una escena snapshot.
5. `RenderState` convierte esa escena en draw calls.
6. `egui` dibuja el panel de debug y el viewport.

## Contrato de escena

La escena es un snapshot, no un estado mutable permanente.

Se compone de objetos con:
- mesh
- transform
- opcionalmente una `TerrainPatch`

Meshes actuales:
- `Terrain`
- `Water`
- `Cube`
- `Grid`
- `Axes`
- `Origin`

## Capas visuales

La escena actual separa estas capas:
- terreno base
- agua
- isla
- grilla
- ejes
- origen
- cubos de prueba

## Sistema cartesiano

La convención base es:
- `Y` arriba
- `Z` hacia adelante
- `X` hacia la derecha

Esto se usa en:
- cámara
- grilla
- gizmo de ejes
- interpretación del mundo

