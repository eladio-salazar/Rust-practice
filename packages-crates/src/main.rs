// Punto de entrada del paquete, desde aqui se probara el llamado a crates de objetos
// como figuras geometricas y vehiculos, solo para afianzar el manejo de paquetes, 
// crates, modulos y uso de use y mod
use packages_crates::fecha_hora::calendarios::Calendario;


fn main() {
    let calendario = Calendario::now();
    println!("Hola desde main.rs");
    println!("Calendario creado: {:?}", calendario)
}
