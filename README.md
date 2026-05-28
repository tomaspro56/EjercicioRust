# EjercicioCiclos — Inventario en Rust

Ejercicio de programación en **Rust** que demuestra el uso de **ciclos `for` anidados** para calcular el valor total de un inventario organizado por categorías de productos.

Desarrollado para la materia de **Lógica de Programación** — ITM 2026.

---

## Tabla de contenidos

- [Descripción](#descripción)
- [Objetivo pedagógico](#objetivo-pedagógico)
- [Estructura del proyecto](#estructura-del-proyecto)
- [Datos del inventario](#datos-del-inventario)
- [Explicación del código](#explicación-del-código)
- [Conceptos de Rust demostrados](#conceptos-de-rust-demostrados)
- [Ejemplo de salida](#ejemplo-de-salida)
- [Cómo ejecutar](#cómo-ejecutar)
- [Requisitos](#requisitos)
- [Integrantes](#integrantes)

---

## Descripción

El programa modela un inventario de una tienda con tres categorías de productos. Para cada categoría recorre sus artículos, calcula el valor individual (`precio × cantidad`), acumula un subtotal por categoría y, al finalizar todas las categorías, muestra el valor total del inventario.

El ejercicio simula una lógica real de negocios (valuación de stock) usando únicamente las estructuras básicas del lenguaje: arrays, tuplas y ciclos `for`.

---

## Objetivo pedagógico

Este ejercicio busca que el estudiante comprenda y aplique:

1. **Ciclos anidados** — un bucle externo recorre categorías; uno interno recorre los productos de cada categoría.
2. **Destructuring de tuplas** — extraer múltiples valores de una tupla en una sola línea.
3. **Acumuladores** — uso de variables mutables para sumar parciales y totales.
4. **Tipos de datos en Rust** — enteros (`i32`), cadenas (`&str`) y arrays de longitud fija.
5. **Salida formateada** — uso de la macro `println!` con argumentos posicionales.

---

## Estructura del proyecto

```
EjercicioRust/
├── README.md
└── inventario/
    ├── Cargo.toml        ← Metadatos del paquete (nombre, versión, edición)
    ├── Cargo.lock        ← Versiones exactas de dependencias (generado automáticamente)
    └── src/
        └── main.rs       ← Código fuente principal
```

---

## Datos del inventario

El inventario contiene **3 categorías** con **3 productos** cada una. Cada producto se representa como una tupla `(nombre, precio, cantidad)`:

| Categoría | Producto  | Precio ($) | Cantidad (u) | Valor ($)  |
|-----------|-----------|------------|--------------|------------|
| Bebidas   | Agua      | 2.000      | 10           | 20.000     |
| Bebidas   | Jugo      | 3.500      | 5            | 17.500     |
| Bebidas   | Gaseosa   | 4.000      | 8            | 32.000     |
| **Subtotal Bebidas** |  |            |              | **69.500** |
| Snacks    | Papas     | 2.500      | 12           | 30.000     |
| Snacks    | Galletas  | 3.000      | 6            | 18.000     |
| Snacks    | Maní      | 1.800      | 20           | 36.000     |
| **Subtotal Snacks** |  |            |              | **84.000** |
| Aseo      | Jabón     | 5.000      | 7            | 35.000     |
| Aseo      | Champú    | 12.000     | 4            | 48.000     |
| Aseo      | Cepillo   | 3.500      | 15           | 52.500     |
| **Subtotal Aseo** |  |            |              | **135.500**|
| **TOTAL GENERAL** |  |            |              | **289.000**|

---

## Explicación del código

### 1. Estructura de datos — array de tuplas anidadas

```rust
let inventario = [
    ("Bebidas", [("Agua", 2000, 10), ("Jugo", 3500, 5), ("Gaseosa", 4000, 8)]),
    ("Snacks",  [("Papas", 2500, 12), ("Galletas", 3000, 6), ("Maní", 1800, 20)]),
    ("Aseo",    [("Jabón", 5000, 7), ("Champú", 12000, 4), ("Cepillo", 3500, 15)]),
];
```

`inventario` es un array de 3 elementos. Cada elemento es una tupla de dos partes:
- `&str` — nombre de la categoría.
- `[(&str, i32, i32); 3]` — array de 3 productos, donde cada producto es `(nombre, precio, cantidad)`.

### 2. Acumulador del total general

```rust
let mut total_general = 0;
```

Se declara `mut` porque su valor cambia en cada iteración del ciclo externo.

### 3. Ciclo externo — recorre categorías

```rust
for (categoria, productos) in inventario {
    println!("=== {} ===", categoria);
    let mut subtotal_categoria = 0;
    // ...
}
```

El *destructuring* `(categoria, productos)` extrae simultáneamente el nombre de la categoría y su array de productos en cada vuelta.

### 4. Ciclo interno — recorre productos

```rust
for (nombre, precio, cantidad) in productos {
    let valor_producto = precio * cantidad;
    println!("  {} -> ${} x {} unidades = ${}", nombre, precio, cantidad, valor_producto);
    subtotal_categoria = subtotal_categoria + valor_producto;
}
```

Por cada producto se calculan las unidades y se van sumando al subtotal de esa categoría.

### 5. Cierre de cada categoría y acumulación del total

```rust
println!("  Subtotal {}: ${}", categoria, subtotal_categoria);
println!();
total_general = total_general + subtotal_categoria;
```

Al terminar el ciclo interno se imprime el subtotal y se suma al total general.

### 6. Resultado final

```rust
println!("VALOR TOTAL DEL INVENTARIO: ${}", total_general);
```

---

## Conceptos de Rust demostrados

| Concepto | Descripción | Dónde se usa |
|---|---|---|
| Arrays | Colección de tamaño fijo en tiempo de compilación | `inventario`, `productos` |
| Tuplas | Agrupación de valores de distintos tipos | Cada producto `(&str, i32, i32)` |
| Ciclos `for` anidados | Iteración sobre colecciones dentro de otras | Bucle externo + interno |
| Destructuring | Desempaquetar tuplas en variables | `for (categoria, productos)` y `for (nombre, precio, cantidad)` |
| Variables mutables (`mut`) | Permite modificar el valor de una variable | `total_general`, `subtotal_categoria` |
| Macro `println!` | Salida formateada a consola | Toda la impresión de resultados |
| Aritmética entera | Multiplicación y suma de enteros | `precio * cantidad`, acumuladores |

---

## Ejemplo de salida

Al ejecutar el programa se obtiene la siguiente salida en consola:

```
=== Bebidas ===
  Agua -> $2000 x 10 unidades = $20000
  Jugo -> $3500 x 5 unidades = $17500
  Gaseosa -> $4000 x 8 unidades = $32000
  Subtotal Bebidas: $69500

=== Snacks ===
  Papas -> $2500 x 12 unidades = $30000
  Galletas -> $3000 x 6 unidades = $18000
  Maní -> $1800 x 20 unidades = $36000
  Subtotal Snacks: $84000

=== Aseo ===
  Jabón -> $5000 x 7 unidades = $35000
  Champú -> $12000 x 4 unidades = $48000
  Cepillo -> $3500 x 15 unidades = $52500
  Subtotal Aseo: $135500

VALOR TOTAL DEL INVENTARIO: $289000
```

---

## Cómo ejecutar

```bash
# 1. Clonar o descargar el repositorio
git clone <url-del-repositorio>

# 2. Entrar al directorio del proyecto
cd EjercicioRust/inventario

# 3. Compilar y ejecutar con Cargo
cargo run
```

Para compilar sin ejecutar:

```bash
cargo build
```

Para ejecutar las verificaciones del compilador sin generar binario (más rápido):

```bash
cargo check
```

---

## Requisitos

- **Rust** 1.75 o superior (edición 2024).
- **Cargo** (incluido con la instalación estándar de Rust).

Instalar Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

En Linux/macOS:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Integrantes

**Grupo 2 — Lógica de Programación · ITM 2026**

| Nombre                  |
|-------------------------|
| Tomás Pérez             |
| Yesica Caro             |
| Juan Manuel Castaño     |
| Daniel Herrera          |
