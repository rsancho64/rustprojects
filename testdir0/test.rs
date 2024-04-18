fn main() {
    println!("main comienza.");
    saluda("ramon");    
    let resultado = sumame(3,4);
    println!("el resultado es {}", resultado);    
    println!("main acaba.");    
}

fn saluda(persona: &str){
    println!("hola {}", persona);
}

fn sumame(a: i32, b: i32) -> i32 {
    return a + b;
}