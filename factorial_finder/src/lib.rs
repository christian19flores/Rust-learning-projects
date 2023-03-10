use std::error::Error;

pub struct Config {
    pub num: i32,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let num = match args.next() {
            Some(arg) => arg.parse::<i32>().unwrap(),
            None => return Err("Didn't get a number to find factorial")
        };

        Ok(Config { num })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let result = factorial(config.num);

    println!("Factorial of {0}: {result}", config.num);
    
    Ok(())
}

pub fn factorial(num: i32) -> i32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}