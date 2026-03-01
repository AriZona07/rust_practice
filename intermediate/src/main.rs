fn main() {
    // Funciones
    println!("Funciones:");

    /* 
     * Las funciones se crean con
     * fn [nombre de la funcion]([Parámetros de la función]){}
     * fn greet(name: &str){}
     * 
     * Las funciones sirven para poder reutilizar bloques de código
     * Igual sirven para hacer más legible y optimizado el código
    */
    fn greet(name: &str) { // Pide un parámetro "name" de tipo string
        println!("Hello, {}!", name);
    }
    /* 
     * Para llamar a una función se usa:
     * [nombre de la función]([parámetro]);
     * greet("John");
    */
    greet("John");


    // Funciones con valores Retornables
    println!("Funciones con valores retornables:"); // Retorna significa Devolver*

    // Usa el símbolo  -> para mostrar el tipo del valor que retornará
    fn add(a: i32, b: i32) -> i32 { // Pide un parámetro "a" y "b", retorna un entero
        return a + b; // Retorna la suma de a + b
    }

    // La variable "sum" obtiene el valor que retorne la función
    let sum = add(3, 4); // sum obtiene el valor de 3+4 (7)
    println!("Sum is: {}", sum); // Imprime el valor que retornó la función

    // El siguiente bloque es otra forma posible de escribir el anterior bloque de código
    /*
        fn add(a: i32, b: i32) -> i32 {
        a + b // Al no poner "return" no se pone ";" tampoco
        }

        let sum = add(3, 4);
        println!("Sum is: {}", sum);
    */

    
}
