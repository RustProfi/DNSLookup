extern crate rand;
use rand::{Rng};

enum MathStuff {
    DIRAC,
    SQRT(f32),
    ADD(i32, i32),
}

impl MathStuff {
    fn print(&self) {
        match self {
            MathStuff::DIRAC => println!("Ein Dirac ist {}", self.get_result()),
            MathStuff::SQRT(i) => println!("Die Wurzel aus {} ist {}",i, self.get_result()),
            MathStuff::ADD(i,j) => println!("{} blus {} ist {}", i, j, self.get_result())
        }
    }

    fn get_result(&self) -> f32 {
        match self {
            MathStuff::DIRAC => 1.0,
            MathStuff::SQRT(i) => i.sqrt(),
            MathStuff::ADD(i,j) => (i + j) as f32,
        }
    }
}

fn main() {
    let dirac = MathStuff::DIRAC;
    let wurzel = MathStuff::SQRT(4.0);
    let add = MathStuff::ADD(1,1);

    dirac.print();
    wurzel.print();
    add.print();

    for _ in 1..10 {
        match math_generator() {
            Some(value) => value.print(),
            None => println!("Der None Fall ist eingetreten.")
        }
    }

    let mut diraccount = 0;
    for _ in 1..1000 {
        if let Some(MathStuff::DIRAC) = math_generator() {
            diraccount+=1;
        }
    }
    println!("Unter 1000 durchläufen wurden {} Diracs erzeugt :)", diraccount);
}

fn math_generator() -> Option<MathStuff> {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0,4) {
        0 => Some(MathStuff::DIRAC),
        1 => Some(MathStuff::ADD(rng.gen_range(0,100),rng.gen_range(0,100))),
        2 => Some(MathStuff::SQRT(rng.gen_range(0,1000) as f32)),
        _ => None
    }
}
