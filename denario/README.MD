# Denario

Una aplicación para manejar las cuentas personales.

La sección principal que sería la pantalla de bienvenida donde se puedan recorrer los registros, agruparlos por categorías, recorrerlos por tiempo, ver créditos, valor de dolar, cambios en esos valores, etc.

Tiene 3 secciones

+ Registros
+ Categorías
+ Créditos

Tiene una adición que es un scraper para tomar el valor del dólar. Está desarrollado en argentina y es un factor bastante decisivo. Tiene una opción para convertir esos valores al valor del dólar con relación al cambio no oficial - lamentablemente -.


Cada uno de estas secciones tiene un CRUD (Alta, Baja, Modificación)

+ Leer todos
+ Leer uno
+ Crear
+ Actualizar
+ Borrar

# Setup / Instalación

1. Renombrar el archivo .env.template a .env
2. Modificar el archivo .env con los datos del hosting y el nombre de la base de datos.
3. Ejecutar la aplicación con el comando _cargo r_ y abrir el navegador para crear la base de datos y crear los seeds
4. Para crear db y popular las tablas ingresando a http://[_YOUR_HOST_]:[_YOUR_PORT_]/seeds/setup
5. remover esta ruta del archivo main.rs
6. 

## Todo

// BACKEND
[] Crear la estructura básica de la applicación
    [x] Actix
    [] _Features (Una carpeta con mod.rs / models (modelo) / features (routes + controller) )_ (OPCIONAL)
    [] Backup de esta estructura para futuros proyectos sin importar de que forma se almacene la información en una base de datos
    [] Realizar un sistema de login, usuarios y roles. ¿Por qué? Porque el frontend se puede acceder desde toda la red.
[x] Crear la estructura de la app con sus cruds
    [x] Registros
    [x] Categorías
    [x] Créditos
    [x] Dolar (prioridad baja)
[x] Relizar una feature para acceder a los balances
[] Agregar un control para que en caso de que el mes anterior haya quedado saldo a favor lo cargue como un ingreso del mes siguiente.


// FRONT END
[] hacer la pantalla principal
    [] Mostrar los registros
    [] Mostrar los subtotales del mes
        [] Saldo a favor del mes anterior
        [] Diferencia entre el ingreso y el egreso
        [] Mostrar un gráfico con los gatos en el mes de las categorías que figuran en él con los valores del total gastado.
    [] Poder navegar a meses anteriores.
    [] Crear categorías especiales desde el código para calcular los créditos
    [] Poder navegar por las categorías para ver el total gastado en un rango de tiempo.
    [] Agregar un control para actualizar el precio del dolar.

[Migración de la base de datos]

### Tecnologías
- Rust
  - Actix
  - SQLITE
- HTML
- Vuejs
- CSS

