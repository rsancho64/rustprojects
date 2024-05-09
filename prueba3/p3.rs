fn main() {

    let tuple_e: (char, i32, bool) = ('E', 5, true);
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );

    let tuple_e: (char, i32, bool) = ('S', 20, true);
    println!(
        "Es '{}' la {} letra del alfabeto? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );

    let s: &str = "Hola";
    println!(
        "En la cadena '{}' la letra 0 es {}", s, s.is_ascii() // s.as_bytes()[0].to_ascii() as u8
    );
    

}