fn main() {
    println!("\n   ----- Borrowing en Rust -----   \n");

    let saludo = String::from("Hola Mundo");

    let refenrencia = &saludo; // --> parámetro de referencia prestado 

    print!("Original: {}", saludo);
    print!("\nReferencia: {}", refenrencia);

    // Uso de Borrowing y error
    // Borrow Checking
    std::mem::drop(saludo);

    // Error: refrencia inválida, saludo eleminado de memoria, no se puede acceder a la referencia
    // En C++, C  esto si compila, en Rust no se permite
    // print!("Rust no ejecuta en compilación: {}", refenrencia);

    print!("\n \n********************** \n \n");

    let mut saludo_mut = String::from("Hola Mundo");

    let refenrencia_mut = &mut saludo_mut; // --> parámetro de referencia prestado mutable

    refenrencia_mut.push_str(", cruel de cerdos Capitalistas.\n");

    // No puedo usar la variable original mientras tenga una referencia mutable activa
    // Ni para lectura ni para escritura, Rust no permite esto, C++ y C si lo permiten
    // print!("Original mutable: {}", saludo_mut);

    print!("Referencia mutable: {}", refenrencia_mut);

    // Una vez que la referencia mutable deja de existir, puedo usar la variable original
    print!("Original mutable: {}", saludo_mut);

    print!("\n")
}
