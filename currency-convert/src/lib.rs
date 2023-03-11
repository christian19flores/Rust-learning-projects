use std::error::Error;
use std::io;

pub struct Config {
    pub amount: f32,
    pub cur_start: String,
    pub cur_end: String,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let mut input_amount = String::new();

        let mut input_cur_start = String::new();

        let mut input_cur_end = String::new();

        // get amount
        println!("Enter currency amount:");

        io::stdin()
            .read_line(&mut input_amount)
            .expect("Failed to read input");

        let amount = input_amount
            .trim()
            .parse::<f32>()
            .expect("That's not a number");

        // get start currency
        println!("Enter start currency:");

        io::stdin()
            .read_line(&mut input_cur_start)
            .expect("Failed to read input");

        let cur_start = input_cur_start.as_str().trim().to_string();

        // get end currency
        println!("Enter end currency:");

        io::stdin()
            .read_line(&mut input_cur_end)
            .expect("Failed to read input");

        let cur_end = input_cur_end.as_str().trim().to_string();

        Ok(Config {
            amount,
            cur_start,
            cur_end,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let result = self.convert();
    
        println!(
            "Convert {1} {3} to {2}: \n {0}",
            result, self.amount, self.cur_end, self.cur_start
        );
    
        Ok(())
    }
    
    pub fn convert(&self) -> f32 {
        match self.cur_start.as_str() {
            "usd" => {
                if self.cur_end == "gbp" {
                    self.amount * 0.85
                } else {
                    panic!("F's in the chat");
                }
            }
            "gbp" => {
                if self.cur_end == "usd" {
                    self.amount * 1.15
                } else {
                    panic!("F's in the chat");
                }
            }
            _ => todo!(),
        }
    }
}