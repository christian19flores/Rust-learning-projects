use std::error::Error;
use std::io;

pub struct Config {
    triangle_size: i32,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let mut input_size = String::new();

        println!("Select a triangle size: ");
        io::stdin()
            .read_line(&mut input_size)
            .expect("Failed to get triangle size");

        let triangle_size = input_size
            .trim()
            .parse::<i32>()
            .expect("That's not a number");

        Ok(Config { triangle_size })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut triangle_row = String::new();

        for n in 1..=self.triangle_size {
            triangle_row = (0..n).map(|_| "*").collect::<String>();
            println!("{}", triangle_row);
        }

        Ok(())
    }
}
