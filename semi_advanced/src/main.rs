fn main() {
    println!("Cadenas de texto:");
    /*
     * Hay 2 tipos de cadenas de texto
        * &str
        * String
     * 
     */

    let mut hello = String::from("Hello, ");
    // Igual se puede escribir así:
    // let mut hello = "Hello, ".to_string();

    hello.push('w'); // Añade un char al String
    hello.push_str("orld!"); // Añade un &str al String

    println!("{}", hello);


    // Concatenar
    println!("\nConcatenación:");

    // Variables con texto separado
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");

    // Variables con texto concatenado
    let result = format!("{} {} {}", s1, s2, s3); // Concatena las variables dentro de la variable "result"
    println!("{}", result); // Imprime result

    /* Alternativa para concatenar

        // Variables con texto separado
        let s1 = String::from("Hello");
        let s2 = String::from("World!");
        let s3 = String::from("What a beautiful day!");

        // Se concatenan en "result"
        let result = s1 + " " + &s2 + " " + &s3;
        // Imprime result
        println!("{}", result); 

    */


    // Obtener el largo de una cadena
    println!("\nObtener el largo de una cadena:");

    let name = String::from("John");
    println!("Length: {}", name.len()); // 4


    // Propiedad para manejo de memoria
    println!("\nPropiedad para manejo de memoria");

    let a = String::from("Hello"); // Se define "a"
    let b = a; // "b" toma el valor de "a", ahora "a" ya no existe

    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value


    // Tipos simple como números, char y booleanos sólo se copian, no cambian su propiedad.
    let a: i32 = 5;
    let b: i32 = a;
    println!("a = {}", a);  // Funciona
    println!("b = {}", b);  // Funciona


    // Si sólo quieres copiar en tipos String
    // Usa .clone()
    let a = String::from("Hello");
    let b = a.clone(); // Ahora ambos tienen el mismo valor

    println!("a = {}", a);  // Funciona
    println!("b = {}", b);  // Funciona
}
