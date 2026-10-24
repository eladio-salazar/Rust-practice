// Definicion de Struct Calendario, enum dia_letras

use chrono::{Datelike, Local, Duration};

#[derive(Debug, Clone)]
pub struct Calendario {
    year : i32,
    month : u32,
    day : u32,
}

impl Calendario{
    //Funciones de instanciacion
    pub fn now() -> Self {
        let ahora = Local::now();
        Calendario{
            year : ahora.year(),
            month : ahora.month(),
            day : ahora.day(),
        }
    }
    pub fn new(year:i32, month:i32, day:i32) -> Option<Self>{
        if month < 1 || month > 12 || day < 1 || day > 31{
            return None;
        }
        Some(Calendario{
                year : year,
                month : month as u32,
                day : day as u32,
            })
    }
    // Getters
    pub fn year(&self) ->i32 {
        self.year
    }
    pub fn month(&self) ->u32 {
        self.month
    }
    pub fn day(&self) ->u32 {
        self.day
    }

    // Funciones de edicion de calendario
    
    pub fn change_day(&mut self, dias:i64) {
        println!("Cambio de dias pendiete de implementar {dias}");
    }


}


#[derive(Debug, Clone)]
pub enum DiaLetras {
    Lunes,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado, 
    Domingo,
}