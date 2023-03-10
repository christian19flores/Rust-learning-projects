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

        let mut cur_start = String::new();

        let mut cur_end = String::new();


        // get amount
        println!("Enter currency amount:");

        io::stdin()
            .read_line(&mut input_amount.parse::<f32>().unwrap())
            .expect("Failed to read input");

        // get start currency
        println!("Enter start currency:");

        io::stdin()
            .read_line(&mut cur_start);
            .expect("Failed to read input");

        // get end currency
        println!("Enter end currency:");

        io::stdin()
            .read_line(&mut cur_end)
            .expect("Failed to read input");        

        Ok(Config {  })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let result = convert(config.amount, config.cur_start, config.cur_end);

    println!("Convert {0.0} {config.start} to {config.cur_end}: \n {0.0}", config.amount, result);
    
    Ok(())
}

pub fn convert(amount: f32, cur_start: String, cur_end: String) -> f32 {
    
}