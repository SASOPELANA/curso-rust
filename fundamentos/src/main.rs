// Constantes en Rust
// Debe declararse con la palabra clave const y debe tener un tipo de dato explícito.
// Debe ser en mayúsculas y con guiones bajos para separar palabras.
// A nivel global

const PI: f64 = 3.14;

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

    println!("\n   ----- Variables Inmutables - Variables Mutables - Constantes -----\n");

    let mut contador = 0;

    contador = contador + 1;

    println!("Contador: {}", contador);

    println!("\n   ----- Constantes -----\n");
    println!("El valor de PI es: {}", PI);

    valor_pi();
    print!("\n");
    let n = get_num(5);

    println!("El valor de n es: {}", n);
}

fn valor_pi() {
    println!("El valor de PI es: {}", PI);
}

fn get_num(x: i32) -> i32 {
    return x;
}
