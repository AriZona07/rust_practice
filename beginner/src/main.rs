fn main() {
    // variables (mutables)
    println!("Variables mutables"); 

    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);


    // placeholder
    println!("\nPlaceholder"); 

    let is_logged_in: bool = true;
    println!("User logged in? {}", is_logged_in);


    // constantes
    println!("\nConstantes"); 
    // const MINUTES_PER_HOUR = 60; // error: se necesita especificar el tipo
    // Las constantes se escriben en mayúscula por convención
    const BIRTHYEAR: i32 = 1980;
    const MINUTES_PER_HOUR: i32 = 60;

    println!("Birth year: {}", BIRTHYEAR);
    println!("Minutes per hour: {}", MINUTES_PER_HOUR);


    // Operadores y operadores lógicos
    println!("\nOperadores Lógicos");
      /* Operadores lógicos y operadores iguales a javascript
        &&  ==  AND  == true if both values are true
        ||  ==  OR   == true if at least one is true
        !   ==  NOT  == inverts the boolean value
     */

    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    
    // Condicionales
    println!("\nCondicionales"); 

    let is_logged_in: bool = true;
    if is_logged_in {
        println!("Welcome back!");
    } else {
        println!("Please log in.");
    }

    // Match/Swicth
    println!("\nMatch: 1");
    // Match es como el Switch en C#
      let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."), // "_" es como el "default"
    }

    // Otro ejemplo
    println!("\nMatch: 2");
    
    match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
    }
}