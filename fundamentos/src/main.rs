fn main() {
    println!("\n   ----- Fundamentos de Rust -----\n");

    // Declarar Variables en Rust
    // Las variables en Rust declaradas con let son inmutables por defecto,
    // lo que significa que no
    // se pueden cambiar una vez que se les asigna un valor.

    let edad = 30;

    println!("Mi edad es:{}", edad);

    // Binding de variables en Rust

    let nombre = String::from("Juan Luna");

    let name = nombre;

    // Clonando la variable nombre para crear una nueva variable name
    // Esto permite que tanto nombre como name tengan su propia copia de los datos,
    // let name = nombre.clone();

    println!("{}", name);

    // Esto no es posible ya que nombre se movió a name, por lo que nombre ya no es válido
    // println!("{}", nombre);
}
