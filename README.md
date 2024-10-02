# MinecraftDiorama

![Screenshot 2024-10-01 204142](https://github.com/user-attachments/assets/eb25c21d-52b5-42c5-8ada-e3dfaf692cdb)

Este proyecto es una representación de un diorama de Minecraft renderizado utilizando ray tracing implementado en Rust. El objetivo principal es simular efectos avanzados de iluminación, sombras, reflejos y refracciones para lograr un estilo gráfico más realista que el del Minecraft original.

## Video

Puedes ver un video del diorama [aquí](https://youtu.be/aWzC6-2vB8s)

## Características 

- Renderizado con Ray Tracing: Simulación precisa de la propagación de la luz en la escena para obtener sombras suaves y reflejos realistas.
- Escenario de Minecraft: Recreación de un diorama de Minecraft con bloques y entidades clásicas del juego.
- Sombras dinámicas: Las sombras de los objetos varían en función de la posición de la fuente de luz.
- Reflejos y refracciones: Superficies reflectantes y transparentes que interactúan con la luz.
- Eficiencia de Rust: Aprovechamos la velocidad y el control de memoria de Rust para ejecutar los cálculos de ray tracing de manera eficiente.
- Movimiento de Camara: Movimientos Pitch, Yaw y Acercamiento.
- Movimiento de luz: Se puede modificar la posición de la luz que simula el sol, para simular el ciclo del día. 

## Instalación
Clona este repositorio:
```
git clone https://github.com/usuario/minecraft-raytracing.git
cd minecraft-raytracing
```

Instala las dependencias y construye el proyecto:
```
cargo build --release
```

## Uso
Para renderizar el diorama de Minecraft, simplemente ejecuta el proyecto:

```
cargo run --release
```

- Para movimeinto Pitch y Yaw, utiliza WASD.
- Para acercamiento de camara, utiliza las flechas arriba y abajo. 
- Para movimiento de sol, utiliza las felchas derecha e izquierda.

## Contribuciones 

¡Las contribuciones son bienvenidas! Si deseas mejorar este proyecto, abre un pull request o crea un issue con tus sugerencias.


