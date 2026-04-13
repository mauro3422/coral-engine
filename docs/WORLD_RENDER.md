# Mundo y render

## Terreno

El terreno base se genera alrededor del origen y se usa como suelo estable del mundo.

## Agua

El agua es una capa separada del terreno:
- tiene su propia malla
- tiene su propio pipeline
- puede dibujarse plana o con olas

Las olas deforman solo el agua, no el terreno.

## Isla

La isla actual es una forma de prueba para validar:
- lectura de escala
- relación con el agua
- claridad visual de capas

No es todavía un sistema de terreno avanzado.

## Entidades

Las entidades son objetos lógicos que el motor puede actualizar y renderizar.

La física actual sincroniza sus cuerpos con entidades por índice.

## Limitaciones actuales

- no hay colisiones reales
- no hay materiales formales
- no hay editor de escena
- el agua sigue siendo visual, no física

