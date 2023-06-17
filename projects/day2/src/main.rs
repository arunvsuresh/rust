fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    {
        // shadowing of x, useful if want to reuse var name for chaining commands/methods
        let x = x + 1;
        println!("the value of x is: {x}");
    }
    let x = 7;
    println!("The value of x is: {x}");
    // more shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("the value of spaces is: {spaces}");

    // INTS AND FLOATS
    let guess: u32 = "21".parse().expect("Not a number!");
    println!("{}", guess);
    let x: u32 = 64;
    println!("{}", x);
    let y: f32 = 2.0;
    println!("{}", y);

    // CHARS
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // COMPOUND TYPES 
    // (tuples)
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    
    // arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];


    

}
