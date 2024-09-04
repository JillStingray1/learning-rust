#[derive(Debug)]
pub enum Inputs {
    Number(i64),
    Add,
    Subtract,
    Multiply,
}

pub struct Calculator {
    pub display_value: String,
    pub inputs: Vec<Inputs>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            display_value: String::from(""),
            inputs: vec![],
        }
    }

    pub fn add_input(&mut self, input: Inputs) {
        self.inputs.push(input)
    }

    pub fn evaluate(&mut self) {
        use Inputs::*;
        let mut number_1 = 0;
        let mut number_2 = 0;
        while self.inputs.len() > 0 {
            let mut digit = 1;
            number_1 = match self.inputs.pop().unwrap() {
                Number(x) => number_1 + x * digit,
                Add => {
                    number_2 += number_1;
                    0
                }
                Subtract => todo!(),
                Multiply => todo!(),
            }
        }
    }
}

fn do_operation(operation: Inputs, num_1: i64, num_2: i64) -> i64 {}
