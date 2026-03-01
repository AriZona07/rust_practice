fn main() {
    // Loops
    println!("Loops en Rust:");
    /* 
     * Nunca decomentes este bloque de código
     * Este bloque produce un bucle infinito
    loop {
        println!("This will repeat forever!"); // "ctrl + c" para parar
    }
     */

    let mut count: i32 = 1;
    loop { //Este loop imprime "Hello World!" 3 veces
        println!("Hello World!");
        if count == 3 {
            break;
        }
        count += 1;
    }

    // Loop con variable (contador mejorado)
    println!("\nContador mejorado (Loop con variable):");

    let mut count: i32 = 1;
    let result: i32 = loop {
        println!("Hello!");

        if count == 3 {
            break count; // para el loop cuando count retorne 3
        }

        count += 1;
    }; // result suma +1 en cada loop aunque no esté explicito, funciona como contador
    
    // Imprime cuantas veces se repitió el loop
    println!("The loop stopped at: {}", result);


    // Bucle While
    println!("\nWhile loop:");

    let mut count: i32 = 1;
    while count <= 5 { // Mientras count sea menor que o igual a 5
        println!("Count: {}", count); 
        count += 1; // Aumenta el contador
    }

    // Ejemplo de condicion falsa en bucles while
    let count: i32 = 10;
    while count <= 5 { // Mientras count sea menor que o igual a 5
        println!("This won't be printed.");
    } // Este bucle nunca se ejecuta, porque count es mayor que 5


    // Saltar un valor
    println!("Saltar un valor:");
    let mut num: i32 = 1;

    while num <= 10 {
        // El if + continue sirve para saltar un valor
        if num == 6 { // Revisa si num es igual a 6
            num += 1; // Si es igual a 6, le suma 1
            continue;
        } // El if + continue se pone antes de que se imprima num para que salte correctamente el 6

        println!("Number: {}", num);
        num += 1; // Aumenta el contador
    }


    // Loops for
    println!("Loops for:");
    /*
     * Los loops for no necesitan poner un "i++"
     * Tampoco se necesita declarar manualmente, Rust lo hace en automático
     * El limite superior del bucle funciona como en matemáticas
     * Esto significa que el límite superior no se incluye, sino el inmediato inferior
     * i in (limite inferior)..(limite superior){}
     * i in 1..6{}
    */

    for i in 1..6 { // El bucle toma los números: 1, 2, 3, 4, y 5
        println!("i is: {}", i); // Imprime el contador "i"
    }

    /* 
     * Si quieres que se incluya el limite superior
     * Usa ..=
     * 1 in 1..=6{}
    */
    for i in 1..=6 {
        println!("i is: {}", i);
    }
}
