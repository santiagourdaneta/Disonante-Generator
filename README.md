# Disonante-Generator 🎵

Un generador de música algorítmica disonante escrito en **Rust**. Este proyecto crea secuencias MIDI únicas y experimentales utilizando dos métodos principales: un generador puramente aleatorio y un modelo de cadena de Markov.

## ✨ Características Principales

* **Modo Aleatorio:** Genera notas al azar de una escala disonante predefinida, creando composiciones impredecibles.
* **Modo de Cadena de Markov:** Utiliza una secuencia de notas semilla para aprender patrones de transición y generar nuevas melodías basadas en esas reglas.
* **Salida MIDI:** Crea archivos `.mid` compatibles con cualquier software de música o reproductor MIDI.
* **Herramienta de Línea de Comandos (CLI):** Fácil de usar y de personalizar los parámetros de generación (número de notas, modo, nombre del archivo de salida).

## 🚀 Uso

Para usar `Disonante-Generator`, necesitas tener **Rust** y **Cargo** instalados.

1.  **Clonar el repositorio:**

    ```bash
    git clone https://github.com/santiagourdaneta/Disonante-Generator/
    cd Disonante-Generator
    ```

2.  **Generar música en modo aleatorio:**

    Este comando crea un archivo MIDI llamado `disonante_random.mid` con 200 notas aleatorias.

    ```bash
    cargo run -- --notes 200 --output disonante_random.mid
    ```

3.  **Generar música en modo de cadena de Markov:**

    Este comando crea un archivo MIDI llamado `disonante_markov.mid` con 500 notas basadas en patrones de Markov.

    ```bash
    cargo run -- --notes 500 --mode markov --output disonante_markov.mid
    ```

### Opciones de la CLI:

| Opción        | Descripción                                      | Valor por defecto          |
| ------------- | ------------------------------------------------ | -------------------------- |
| `--notes`     | Número de notas a generar.                       | `100`                      |
| `--output`    | Nombre del archivo MIDI de salida.               | `output.mid`               |
| `--mode`      | Modo de generación: `random` o `markov`.         | `random`                   |

## 🛠️ Desarrollo y Pruebas

El proyecto incluye una suite de pruebas para asegurar la correcta funcionalidad de todos sus componentes.

* **Ejecutar las pruebas unitarias y de integración:**

    ```bash
    cargo test
    ```

## 📄 Licencia

Este proyecto está bajo la licencia **MIT**. Para más detalles, consulta el archivo `LICENSE`.

Etiquetas (Tags):
rust, midi, music-generation, generative-art, algorithmic-composition, music, cli-tool
