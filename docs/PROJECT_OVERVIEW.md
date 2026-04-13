# Motor Grafico

Motor Grafico es un prototipo de motor 3D en Rust basado en `wgpu`, `winit` y `egui`.

La idea actual del proyecto es tener una base estable para:
- escena 3D renderizable
- cámara libre
- sistema cartesiano explícito
- capas de UI con prioridad sobre el mundo
- terreno, agua y una isla de prueba
- física simple conectada a entidades

## Estado actual

El proyecto ya tiene una base funcional con estas piezas:
- renderer con pipeline principal, pipeline de agua y pipeline de líneas
- escena por frame
- router de input/UI con capas explícitas
- panel de debug con toggles del viewport
- terreno base centrado en el origen
- agua separada como capa visual propia
- isla de prueba para validar interacción visual
- entidades y física básicas

## Qué se considera estable hoy

- El eje del mundo está definido como `up Y / forward Z / right X`.
- La UI tiene prioridad sobre la cámara y el mundo.
- El viewport de debug puede activar o desactivar terreno, agua, olas e isla.
- El agua y el terreno no dependen del mismo toggle.
- Las olas deforman solo la superficie del agua.

## Qué sigue siendo prototipo

- La física todavía es simple y no tiene colisiones reales.
- La isla es una forma de prueba, no un sistema de mundo completo.
- El agua es visual, no una simulación física completa.
- No existe todavía un editor de escena ni un sistema formal de materiales.

