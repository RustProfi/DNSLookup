extern crate rand;
use rand::{Rng};
use std::io::{self, BufRead, Write};

#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Yellow
}

enum Frucht {
    Apfel(f32, Color),
    Birne(f32, Color),
    Traube(f32, Color)
}


impl Color {
    fn to_string(&self) -> String {
        match self {
            Color::Yellow => String::from("gelbe"),
            Color::Green => String::from("grüne"),
            Color::Red => String::from("rote"),
        }
    }

    fn random_col() -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0,3) {
            0 => Color::Red,
            1 => Color::Green,
            _ => Color::Yellow,
        }
    }
}

impl Frucht {
    fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let multiplikator = match rng.gen_range(0,100) {
            0...20 => 0.0,
            20...50 => 0.5,
            50...80 => 1.0,
            80...94 => 2.0,
            94...97 => 10.0,
            _ => 100.0,
        };

        match rng.gen_range(0,100) {
            0...30 => Frucht::Birne(multiplikator, Color::random_col()),
            30...70 => Frucht::Traube(multiplikator, Color::random_col()),
            _ => Frucht::Apfel(multiplikator, Color::random_col()),
        }
    }

    fn print(&self) {
        match self {
            Frucht::Apfel(anzahl, col) => println!("{} {} Äpfel gezogen", anzahl, col.to_string()),
            Frucht::Birne(anzahl, col) => println!("{} {} Birnen gezogen", anzahl, col.to_string()),
            Frucht::Traube(anzahl, col) => println!("{} {} Trauben gezogen", anzahl, col.to_string()),
        }
    }
}

fn roll(einsatz: f32) -> Option<f32> {
    let frucht = Frucht::generate();
    frucht.print();

    let multi = match frucht {
        Frucht::Apfel(i, _) => i,
        Frucht::Birne(i, _) => i,
        Frucht::Traube(i, _) => i,
    };

    let existance = match frucht {
        Frucht::Traube(_, col) => if col == Color::Yellow {false} else {true},
        Frucht::Birne(_, col) => if col == Color::Green {true} else {false},
        _ => true
    };

    if !existance {
        println!("Diese Früchte gibt es nicht :(");
        None
    }
    else if multi == 0.0 {
        None
    }
    else {
        Some(multi * einsatz)
    }
}

fn get_einsatz() -> f32 {
    print!("Einsatz: ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line = line.replace("\n", "").replace("\r", "");
    line.parse::<f32>().unwrap()
}

fn main() {
    println!("Fruchtlotterie! Es wird gespielt bis alles verzockt ist! Startgeld ist 100");
    let mut balance = 100.0;

    while balance > 0.0 {
        let einsatz = get_einsatz();
        balance -= einsatz;
        match roll(einsatz) {
            Some(value) => {balance += value; println!("{} gewonnen :) Neuer Kontostand: {}", value, balance)},
            None=> println!("Leider verloren :( Neuer Kontostand: {}", balance),
        }
    }

    println!("Alles verzockt!");
}