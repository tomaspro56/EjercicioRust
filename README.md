# EjercicioCiclos — Inventario en Rust

Ejercicio de programación en Rust que demuestra el uso de **ciclos anidados** (`for`) para calcular el valor total de un inventario organizado por categorías.

## Descripción

El programa recorre un inventario de tres categorías de productos, calcula el valor de cada artículo (precio × cantidad), acumula subtotales por categoría y muestra el total general al final.

## Estructura del proyecto

```
EjercicioCiclos/
└── inventario/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Categorías y productos

| Categoría | Productos                    |
|-----------|------------------------------|
| Bebidas   | Agua, Jugo, Gaseosa          |
| Snacks    | Papas, Galletas, Maní        |
| Aseo      | Jabón, Champú, Cepillo       |

## Ejemplo de salida

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

## Conceptos demostrados

- Arrays de tuplas como estructura de datos
- Ciclos `for` anidados con destructuring
- Variables mutables (`mut`) para acumuladores
- Formato de salida con `println!`

## Ejecución

```bash
cd inventario
cargo run
```

**Requisito:** tener [Rust](https://www.rust-lang.org/tools/install) instalado.
