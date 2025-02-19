# Calculadora de Fabricación para SimsCity BuildIt - 19/02/2025 - By: owen-ian - https://github.com/owen-ian/SimsCityBuilditFabricaSimple
==========================================================================================================================================

Este programa está diseñado para ayudarte a gestionar y calcular los materiales necesarios para fabricar productos en el videojuego móvil Sims City: BuildIt.

## Instrucciones de Uso

### Instalación de Rust

1. **Instalar Rust**
   - Si no tienes Rust instalado, puedes hacerlo siguiendo las instrucciones en la página oficial: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Ejecutar el Programa

1. **Ejecutar el Programa**
   - Abre la terminal. Puedes utilizar PowerShell, CMD, o cualquier terminal de tu preferencia.
   - Navega al directorio del proyecto.

2. **Iniciar el Programa**
   - Escribe `cargo run` y presiona Enter.

3. **Seleccionar Producto**
   - Introduce el número del producto que deseas fabricar, tal como aparece a la izquierda del nombre del producto en la lista proporcionada.
   - Puedes añadir la cantidad que deseas fabricar utilizando el formato "NúmeroDeProducto*Cantidad".

   **Ejemplo:** Si deseas fabricar 3 martillos y el código del martillo es 7, deberías escribir `7*3`.

4. **Revisar Materiales Requeridos**
   - El programa te mostrará la cantidad de materiales necesarios. Si el producto requiere otros productos además de materiales, también se indicará.

   **Ejemplo:** Supongamos que deseas fabricar Donas (código 31) y necesitas fabricar 8. Debes escribir `31*8`. El programa te mostrará la siguiente información:
     ```
     Materiales necesarios:
     16 - Textiles
     16 - Semillas
     8 - Especias

     Productos necesarios:
     8 - Bolsa de Harina
     ```
   La receta de Donas es 8 Especias + 8 Bolsas de Harina (cada bolsa requiere 2 Textiles y 2 Semillas). Por lo tanto, necesitas 16 Textiles y 16 Semillas.

El programa está diseñado para indicarte desde el inicio lo que debes fabricar en las industrias y luego en las tiendas, en caso de que el producto requiera otros productos previos.
