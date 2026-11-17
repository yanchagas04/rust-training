use std::any::type_name_of_val;

fn main() {
    //primitive types

    let integer: i8 = 10; // with ype annotation
    // let integer = 10; // without type annotation (applies the default type size)
    // let integer = 10i8;

    println!("{} {}", integer, type_name_of_val(&integer));

    let unsigned_integer: u8 = 10; // with type annotation
    // let unsigned_integer = 10; // without type annotation (applies the default type size)
    // let unsigned_integer = 10u8;

    println!(
        "{} {}",
        unsigned_integer,
        type_name_of_val(&unsigned_integer)
    );

    let float: f32 = 10.0; // with type annotation
    // let float = 10.0; // without type annotation (applies the default type size)
    // let float = 10.0f32;

    println!("{} {}", float, type_name_of_val(&float));

    let boolean: bool = true; // with type annotation
    // let boolean = false; // without type annotation (applies the default type size)
    // let boolean = true; // with type annotation

    println!("{} {}", boolean, type_name_of_val(&boolean));

    let character: char = 'a'; // with type annotation
    // let character = 'b'; // without type annotation (applies the default type size)

    println!("{} {}", character, type_name_of_val(&character));

    //compound types

    let tuple: (i32, f64, u8) = (500, 6.4, 1); // with type annotation
    // let tuple = (500, 6.4, 1); // without type annotation (applies the default type size)
    // let tuple = (500, 6.4, 1);

    println!("{:?} {}", tuple, type_name_of_val(&tuple));

    let (x, y, z) = tuple; // destructuring the tuple into individual variables

    println!("x={} y={} z={} ", x, y, z);

    let array: [i32; 5] = [1, 2, 3, 4, 5]; // with type annotation
    // let array = [1, 2, 3, 4, 5]; // without type annotation (applies the default type size)
    // let array = [3; 5]; // creates an array of length 5, filled with the value 3

    println!("{:?} {}", array, type_name_of_val(&array));
}
