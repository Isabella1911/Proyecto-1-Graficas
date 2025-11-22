# Proyecto-1-Gr-ficas

Video https://youtu.be/CfK23BJaobQ

 Características Principales

Motor de Raycasting 3D completo con texturas en paredes, pisos y techos
Sistema de múltiples niveles con 3 zonas jugables

Sistema de salud con efectos visuales de daño
Sprites interactivos: trabajadores (NPCs) y cofres coleccionables
Audio inmersivo con música de fondo y efectos de sonido
Minimapa en tiempo real mostrando la posición del jugador
Controles fluidos con teclado y mouse

Controles
Tecla/AcciónFunción↑ / WAvanzar↓ / SRetroceder← / →Rotar cámaraAStrafe izquierdaDStrafe derechaMouseRotación horizontal de cámara1 / 2 / 3Seleccionar zona (en menú)ENTERIniciar juegoRRespawnear (Game Over)MVolver al menúESCSalir del juego
 Arquitectura del Proyecto
 Tecnologías Utilizadas

Lenguaje: Rust 
Gráficos: raylib-rs - Bindings de Raylib para Rust
Matemáticas: Vectores y trigonometría con raylib::math

instalación
Prerrequisitos

Rust (última versión estable)

bash   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Dependencias del sistema (Linux/macOS)

bash   # Ubuntu/Debian
   sudo apt install libasound2-dev


# Compilar y ejecutar
cargo run --release
```

# Assets

El juego utiliza los siguientes assets (ubicados en `assets/`):

### Texturas
- `wall_main.png` - Textura principal de paredes
- `wall_light.png` - Paredes con iluminación
- `wall_gate.png` - Puertas entre niveles
- `floor_temple.png` - Textura del piso
- `ceil_temple.png` - Textura del techo

### Sprites
- `enemy.png` - Enemigo espectral (F)
- `worker.png` - Trabajador NPC (T)
- `chest.png` - Cofre coleccionable (C)

### Audio
- `music_background.ogg` - Música ambiental de fondo
- `sfx_hit.wav` - Efecto de daño
- `sfx_chest.wav` - Efecto al abrir cofre

### Mapas
- `maze.txt` - Zona 1
- `maze1.txt` - Zona 2
- `maze2.txt` - Zona 3
