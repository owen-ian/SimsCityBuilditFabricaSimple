use std::collections::HashMap;
use std::io::{self};

const ROJO: &str = "\x1b[31m";
const VERDE: &str = "\x1b[32m";
const AMARILLO: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

fn main() {
    let productos_basicos = vec![
        "Clavos",
        "Tablas",
        "Ladrillos",
        "Cemento",
        "Pegamento",
        "Pintura",
    ];

    let productos_herramientas = vec![
        "Martillo",
        "Cinta Métrica",
        "Pala",
        "Espátula",
        "Escalera",
        "Taladro",
    ];

    let productos_agricolas = vec![
        "Vegetales",
        "Bolsa de Harina",
        "Frutas y Bayas",
        "Crema",
        "Maíz",
        "Queso",
        "Carne de Res",
    ];

    let productos_muebles = vec!["Silla", "Mesa", "Textiles para el Hogar", "Armario", "Sofá"];

    let productos_jardineria = vec![
        "Césped",
        "Árbol Joven",
        "Muebles de Jardín",
        "Cortadora de Césped",
        "Fogata",
        "Gnomo de Jardín",
    ];

    let productos_alimentos = vec![
        "Donas",
        "Batido Verde",
        "Panecillo",
        "Pastel de Queso con Cereza",
        "Yogur Helado",
        "Café",
    ];

    let productos_moda = vec![
        "Ropa",
        "Zapatos",
        "Sombrero",
        "Gorra",
        "Reloj",
        "Trajes de Negocios",
        "Mochila",
    ];

    let productos_especiales = vec![
        "Hamburguesa",
        "Papas Fritas",
        "Pizza",
        "Sándwich de Helado",
        "Papas con Queso",
        "Botella de Limonada",
        "Palomitas",
    ];

    let productos_electronicos = vec![
        "Refrigerador",
        "Lavadora",
        "Microondas",
        "Barbacoa",
        "Sistema de Iluminación",
        "TV",
    ];

    let productos: Vec<&str> = [
        &productos_basicos[..],
        &productos_herramientas[..],
        &productos_agricolas[..],
        &productos_muebles[..],
        &productos_jardineria[..],
        &productos_alimentos[..],
        &productos_moda[..],
        &productos_especiales[..],
        &productos_electronicos[..],
    ]
    .concat();

    let requisitos: HashMap<&str, Vec<(&str, i32)>> = vec![
        ("Clavos", vec![("Metal", 2)]),
        ("Tablas", vec![("Madera", 2)]),
        ("Ladrillos", vec![("Mineral", 2)]),
        ("Cemento", vec![("Mineral", 2), ("Química", 1)]),
        ("Pegamento", vec![("Plástico", 2), ("Química", 1)]),
        (
            "Pintura",
            vec![("Metal", 2), ("Mineral", 1), ("Química", 2)],
        ),
        ("Martillo", vec![("Metal", 1), ("Madera", 1)]),
        ("Cinta Métrica", vec![("Metal", 1), ("Plástico", 1)]),
        ("Pala", vec![("Metal", 1), ("Madera", 1), ("Plástico", 1)]),
        (
            "Espátula",
            vec![("Metal", 2), ("Madera", 2), ("Plástico", 2)],
        ),
        ("Escalera", vec![("Metal", 2), ("Tablas", 2)]), // Tablas = 2 Madera
        (
            "Taladro",
            vec![("Metal", 2), ("Plástico", 2), ("Componentes Eléctricos", 1)],
        ),
        ("Vegetales", vec![("Semillas", 2)]),
        ("Bolsa de Harina", vec![("Semillas", 2), ("Textiles", 2)]),
        ("Frutas y Bayas", vec![("Semillas", 2), ("Árbol Joven", 1)]),
        ("Crema", vec![("Grano", 1)]),
        ("Maíz", vec![("Mineral", 1), ("Semillas", 4)]),
        ("Queso", vec![("Grano", 2)]),
        ("Carne de Res", vec![("Grano", 3)]),
        ("Silla", vec![("Madera", 2), ("Clavos", 2), ("Martillo", 1)]),
        ("Mesa", vec![("Clavos", 2), ("Tablas", 2), ("Martillo", 1)]),
        (
            "Textiles para el Hogar",
            vec![("Textiles", 2), ("Cinta Métrica", 1)],
        ),
        (
            "Armario",
            vec![("Cristal", 2), ("Tablas", 2), ("Pintura", 1)],
        ),
        (
            "Sofá",
            vec![("Textiles", 3), ("Pegamento", 1), ("Taladro", 1)],
        ),
        ("Césped", vec![("Semillas", 1), ("Pala", 1)]),
        ("Árbol Joven", vec![("Semillas", 2), ("Pala", 1)]),
        (
            "Muebles de Jardín",
            vec![("Plástico", 2), ("Textiles", 2), ("Tablas", 2)],
        ),
        (
            "Cortadora de Césped",
            vec![("Metal", 3), ("Componentes Eléctricos", 1), ("Pintura", 1)],
        ),
        (
            "Fogata",
            vec![("Ladrillos", 2), ("Cemento", 2), ("Pala", 1)],
        ),
        ("Gnomo de Jardín", vec![("Cemento", 2), ("Pegamento", 1)]),
        ("Donas", vec![("Especias", 1), ("Bolsa de Harina", 1)]),
        (
            "Batido Verde",
            vec![("Vegetales", 1), ("Frutas y Bayas", 1)],
        ),
        ("Panecillo", vec![("Bolsa de Harina", 2), ("Crema", 1)]),
        (
            "Pastel de Queso con Cereza",
            vec![("Frutas y Bayas", 1), ("Bolsa de Harina", 1), ("Queso", 1)],
        ),
        (
            "Yogur Helado",
            vec![("Especias", 1), ("Bolsa de Harina", 1), ("Crema", 1)],
        ),
        ("Café", vec![("Especias", 1), ("Semillas", 2), ("Crema", 1)]),
        ("Gorra", vec![("Textiles", 2), ("Cinta Métrica", 1)]),
        (
            "Zapatos",
            vec![("Plástico", 1), ("Textiles", 2), ("Pegamento", 1)],
        ),
        (
            "Reloj",
            vec![("Plástico", 2), ("Química", 1), ("Cristal", 1)],
        ),
        (
            "Trajes de Negocios",
            vec![("Textiles", 3), ("Pegamento", 1), ("Cinta Métrica", 1)],
        ),
        (
            "Mochila",
            vec![("Plástico", 2), ("Textiles", 2), ("Cinta Métrica", 1)],
        ),
    ]
    .into_iter()
    .collect();

    println!("Lista de productos:");

    println!(
        "{}BUILDING SUPPLIES STORE:{}     {}HARDWARE STORE:{}                {}FARMER'S MARKET:{}",
        ROJO, RESET, ROJO, RESET, ROJO, RESET
    );
    println!("1.  Clavos                   7.  Martillo                   13. Vegetales");
    println!("2.  Tablas                   8.  Cinta Métrica              14. Bolsa de Harina");
    println!("3.  Ladrillos                9.  Pala                       15. Frutas y Bayas");
    println!("4.  Cemento                  10. Espátula                   16. Crema");
    println!("5.  Pegamento                11. Escalera                   17. Maíz");
    println!("6.  Pintura                  12. Taladro                    18. Queso");
    println!("                                                            19. Carne de Res");
    println!(" ");
    println!(
        "\n{}FURNITURE STORE:{}             {}GARDENING SUPPLIES:{}            {}DONUT SHOP:{}",
        ROJO, RESET, ROJO, RESET, ROJO, RESET
    );
    println!("20. Silla                    25. Césped                     31. Donas");
    println!("21. Mesa                     26. Árbol Joven                32. Batido Verde");
    println!("22. Textiles                 27. Muebles de Jardín          33. Panecillo");
    println!("23. Armario                  28. Cortadora de Césped        34. Pastel de Queso con Cereza");
    println!("24. Sofá                     29. Fogata                     35. Yogur Helado");
    println!("                             30. Gnomo de Jardín            36. Café");
    println!(" ");
    println!(
        "\n{}FASHION STORE:{}               {}FAST FOOD RESTAURANT:{}          {}HOME APPLIANCES:{}",
        ROJO, RESET, ROJO, RESET, ROJO, RESET
    );
    println!("37. Ropa                     44. Hamburguesa                51. Refrigerador");
    println!("38. Zapatos                  45. Papas Fritas               52. Lavadora");
    println!("39. Sombrero                 46. Pizza                      53. Microondas");
    println!("40. Gorra                    47. Sándwich de Helado         54. Barbacoa");
    println!(
        "41. Reloj                    48. Papas con Queso            55. Sistema de Iluminación"
    );
    println!("42. Trajes de Negocios       49. Botella de Limonada        56. TV");
    println!("43. Mochila                  50. Palomitas");
    println!(" ");
    println!("{}Selecciona el producto que deseas elaborar, escribiendo 'NumeroDeProducto*Cantidad'{}", AMARILLO, RESET);
    println!("{}Se te informara que Materiales y Productos necesitas elaborar para conseguir lo que deseas{}", AMARILLO, RESET);

    let mut seleccion = String::new();
    io::stdin()
        .read_line(&mut seleccion)
        .expect("Error al leer la entrada");
    let seleccion = seleccion.trim();
    let seleccion_vec: Vec<&str> = seleccion.split(',').map(|s| s.trim()).collect();

    let mut requisitos_totales: HashMap<&str, i32> = HashMap::new();
    let mut productos_requeridos: HashMap<&str, i32> = HashMap::new();

    for sel in seleccion_vec {
        let parts: Vec<&str> = sel.split('*').collect();
        let (producto, cantidad) = if parts.len() == 2 {
            (parts[0], parts[1].parse::<i32>().unwrap_or(1))
        } else {
            (parts[0], 1)
        };

        if let Ok(numero) = producto.parse::<usize>() {
            if numero > 0 && numero <= productos.len() {
                let producto = productos[numero - 1];
                agregar_requisitos(
                    producto,
                    &requisitos,
                    &mut requisitos_totales,
                    cantidad,
                    &mut productos_requeridos,
                );
            } else {
                println!("Número inválido: {}", producto);
            }
        } else {
            if productos.contains(&producto) {
                agregar_requisitos(
                    producto,
                    &requisitos,
                    &mut requisitos_totales,
                    cantidad,
                    &mut productos_requeridos,
                );
            } else {
                println!("Nombre de producto inválido: {}", producto);
            }
        }
    }

    println!("{}Materiales necesarios:{}:", VERDE, RESET);
    for (material, cantidad) in &requisitos_totales {
        if !productos.contains(&material) {
            println!("{}{} - {}{}", VERDE, cantidad, material, RESET);
        }
    }

    println!("{}Productos necesarios:{}:", VERDE, RESET);
    for (producto, cantidad) in &productos_requeridos {
        println!("{}{} - {}{}", VERDE, cantidad, producto, RESET);
    }

    fn agregar_requisitos<'a>(
        producto: &str,
        requisitos: &HashMap<&'a str, Vec<(&'a str, i32)>>,
        requisitos_totales: &mut HashMap<&'a str, i32>,
        cantidad: i32,
        productos_requeridos: &mut HashMap<&'a str, i32>,
    ) {
        if let Some(materiales) = requisitos.get(producto) {
            for &(material, req_cantidad) in materiales {
                let total_requisito = req_cantidad * cantidad;
                *requisitos_totales.entry(material).or_insert(0) += total_requisito;

                if requisitos.contains_key(material) {
                    *productos_requeridos.entry(material).or_insert(0) += total_requisito;
                    agregar_requisitos(
                        material,
                        requisitos,
                        requisitos_totales,
                        total_requisito,
                        productos_requeridos,
                    );
                }
            }
        }
    }
}
