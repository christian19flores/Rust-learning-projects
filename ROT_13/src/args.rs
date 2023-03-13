#[derive(Debug)]
pub struct Args {
    pub option: String,
    pub text: String
}

impl Args {
    pub fn new() -> Self {
        Args {
            option: get_nth_arg(1),
            text: get_nth_arg(2)
        }
    }
}

fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}