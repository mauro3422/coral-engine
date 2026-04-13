# Input y UI

## Objetivo

Evitar que la cámara y la UI peleen por el mismo click.

## Router de entrada

`InputRouter` es la fuente de verdad para:
- capas UI registradas en el frame
- estado `egui` de puntero y teclado
- estado de captura del mundo

### Estados del mundo

- `Idle`: no hay captura activa
- `Armed`: el click del mundo está pendiente
- `Active`: la cámara ya capturó el mouse

## Capas UI

Cada frame, la UI registra rectángulos reales:
- paneles
- ventanas flotantes
- widgets

La decisión de si un click pertenece a UI o al mundo se hace con:
- posición real del cursor
- `pixels_per_point` real de `egui`
- rectángulos registrados por frame

## Regla de prioridad

1. UI primero
2. cámara después
3. mundo al final

Si el cursor toca una capa UI, el mundo no arma captura.

## Viewport de debug

El viewport tiene toggles para:
- `Show terrain`
- `Show water`
- `Show waves`
- `Show island`

Estos toggles afectan capas distintas. No deben apagar otras capas por accidente.

