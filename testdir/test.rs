mod gentilezas;
mod calculos;

fn main() {
    println!("main comienza.");
    gentilezas::saluda("ramon");    
    let resultado = calculos::sumame(3,4);
    println!("el resultado es {}", resultado);    
    println!("main acaba.");    
}

