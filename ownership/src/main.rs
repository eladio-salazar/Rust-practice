// ownership proyect: Solo quiero entender los conceptos del capitulo 4
//

fn main() {
    string_test();
    // Moving data
    let x = 5;
    println!("x:{x}");
    let y = 2 * x;
    println!("x:{x}");
    println!("y:{y}");
    let s1 = give_ownership();
    let s2 = String::from("Hola");
    let s3 = takes_and_gives_back(s2);
    println!("s1:{s1}, s3:{s3}");
    //println!("x:{x}");
}

fn string_test() {
    let mut s = String::from("Hello");
    // let mut s = "Hello";
    s.push_str(", world!");
    println!("{s}");
}

fn give_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
