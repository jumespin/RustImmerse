fn main(){
    let mut a_mascot = "Ferris";
    let a_name = "Rushito";
    let a_memory = "Rushu";
    println!("I Know the current mascot's name is {}, but I prefer {}, or {} ",a_mascot, a_name, a_memory);
    a_mascot = "Jacksito";
    println!("I Know the current mascot's name is {}, but I prefer {}, or {} ",a_mascot, a_name, a_memory);

    let shadow_num: i32 = 5;

    let shadow_num = shadow_num + 5;

    let shadow_num = shadow_num * 2;

    println!("The number is {}", shadow_num);

    let _b_ol: bool = true;
    let _title: &str = "Hola";
    let _age: u8 = 28;

    let _number_64 = 4.0;
    let _number_32: f32 = 5.0;
    let _char: char = 'A';
    let _emoji: char = '💩';

    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    println!("9 / 2 = {} but  9.0 / 2.0 = {}", 9u32 / 2, 9.0/ 2.0);

    println!("Alguna vez en un codigo habias podido imprimir esto {}", _emoji);
}
