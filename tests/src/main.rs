fn main() {
    // Variables containing "mut" are mutable. This means they can be changed later in the code.
    // The equivalent of constants in rust is leaving "mut" out of the variable.
    // Mutable means a variable can be changed, immutable means it can not.
    let mut x = 10;
    println!("The value of x is: {}", x);
    // If I were to use "let" to change a variable that contains, "mut"
    // It would create a new variable instead of changing the other version
    // of itself that was defined earlier.
    x = 5;
    println!("The value of x is: {}", x);
    let sum = x * 2;
    println!("The value of {} * 2 is: {}", x, sum);
    test();
    foo();
    tuple();
    arrays();
    print("Hello, World!");
    print("7474");
}

// Functions
fn test() {
    let spaces = "hello, world!";
    let spacelen = spaces.len(); // Gets amount of characters in a string or something.
    println!("The length of spaces is {}", spacelen);
}

fn foo() {
    let x = 2.0; // f64 | 64-bit
    let y: f32 = 3.0; // f32 | 32-bit
    println!("{} is x, {} is y", x, y);
}

fn tuple() {
    let tup = (429, 37, 4.2);
    let (x, y, z) = tup;

    println!("{}, {}, {}", x, y, z);
}

fn arrays() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{} is the 2nd entry of a.", a[1])
}

// Custom Print function
//     Defining the message parameter as a string
//             |
//            |
//           V
fn print(mes: &str) {
    println!("{}", mes); // <- Print out the message given in the function parameters.
}