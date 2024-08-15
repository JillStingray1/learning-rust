#[derive(Debug)]
pub enum Inputs {
    Number(Vec<i32>),
    Add,
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
        use Inputs::*;
        if self.inputs.len() == 0 {
            match &input {
                Number(_) => self.inputs.push(input),
                _ => return,
            }
        } else {
            let last_token_index = self.inputs.len() - 1;
            match &mut self.inputs[last_token_index] {
                Number(value) => match &input {
                    Number(digit) => value.push(digit[0]),
                    _ => self.inputs.push(input),
                },
                _ => self.inputs.push(input),
            }
        }
    }
}
