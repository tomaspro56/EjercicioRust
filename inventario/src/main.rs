fn main() {
    let inventario = [
        ("Bebidas", [("Agua", 2000, 10), ("Jugo", 3500, 5), ("Gaseosa", 4000, 8)]),
        ("Snacks",  [("Papas", 2500, 12), ("Galletas", 3000, 6), ("Maní", 1800, 20)]),
        ("Aseo",    [("Jabón", 5000, 7), ("Champú", 12000, 4), ("Cepillo", 3500, 15)]),
    ];

    let mut total_general = 0;

    for (categoria, productos) in inventario {
        println!("=== {} ===", categoria);

        let mut subtotal_categoria = 0;

        for (nombre, precio, cantidad) in productos {
            let valor_producto = precio * cantidad;

            println!("  {} -> ${} x {} unidades = ${}", nombre, precio, cantidad, valor_producto);

            subtotal_categoria = subtotal_categoria + valor_producto;
        }

        println!("  Subtotal {}: ${}", categoria, subtotal_categoria);
        println!();

        total_general = total_general + subtotal_categoria;
    }

    println!("VALOR TOTAL DEL INVENTARIO: ${}", total_general);
}