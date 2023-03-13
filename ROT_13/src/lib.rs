use std::error::Error;

pub struct Config {
    pub option: String,
    pub text: String,
}

impl Config {
    pub fn build(option: String, text: String) -> Result<Config, &'static str> {

        Ok(Config {
            option,
            text
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {

        let result = match self.option.as_str() {
            "rot-13" => self.rot13(),
            _ => self.rot13(),
        };

        println!("{result}");

        Ok(())
    }

    pub fn rot13(&self) -> String {
        self.text.chars().map(|c| {
            match c {
                'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
                'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
                _ => c
            }
        }).collect()
    }
}
