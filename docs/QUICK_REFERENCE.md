# 🪸 Coral Engine - Quick Reference

## ¿Qué tengo ahora?
- **1 bloque de agua voxel animado** funcionando
- **UI tipo editor** con sliders en vivo
- **Wireframe naranja** siempre sincronizado
- **Cámara libre** con WASD + mouse
- **GPU instancing** para rendimiento

## Cómo empezar cada sesión
```bash
cargo run --release
```

Si algo raro pasa (cambios no se reflejan):
```bash
cargo clean && cargo run --release
```

## Estructura de carpetas
```
src/
├── ocean/     ← AGUA (el core del motor)
├── core/      ← Motor base (cámara, input, scene)
├── render/    ← GPU rendering
├── ui/        ← Editor UI
├── game/      ← Game state
└── coordinator.rs ← Todo junto
```

## Conceptos rápidos
| Término | Qué es |
|---------|--------|
| Voxel Size | Tamaño de cada cubito |
| Block Size | Tamaño total del bloque de agua |
| Water Layers | Capas de agua (1-4) |
| Wave Height | Amplitud de olas |
| Water Block | El bloque de cubos animados (lo que tenés) |

## Próximas sesiones recomendadas
1. Leer `docs/DEVELOPER_GUIDE.md` para contexto completo
2. Ver el estado de features en la tabla
3. Elegir qué feature sigue del roadmap

## Repo
https://github.com/mauro3422/coral-engine.git