use std::any::Any;
use iced::keyboard::key::{Code, Named};
use iced::widget::{button, column, text, Column, text_input, checkbox, row, Row};
use iced::{Center, Element, Fill, Font, Subscription, Task as Command};
use iced::keyboard::{self, key, Key};
use iced::border::left;

pub fn main() -> iced::Result {
    iced::run("A cool Calculator", Calculator::update, Calculator::view)
    /*let mut equation = String::from("3.3-3.3*3.3-3.3");
    let mut equation_struct:
    */

}


#[derive(Default)]
struct Calculator {
    equation: Equation,
    input_value: String,
    calculated: bool,
    last_dot: bool,
}

#[derive(Debug, Clone)]
struct Equation {
    left: String,
    right: String,
    op: String,
    result: f64,
    full_equation: String,
}

impl Default for Equation {
    fn default() -> Self {
        Equation {
            left: "".to_string(),
            right: "".to_string(),
            op: "".to_string(),
            result: 0.0,
            full_equation: "".to_string(),
        }
    }
}
impl Equation {
    fn new() -> Self {
        Equation {
            left: "".to_string(),
            right: "".to_string(),
            op: "+".to_string(),
            result: 0.0,
            full_equation: "".to_string(),
        }
    }
    fn evaluate(&self) -> f64 {
        self.result
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Clear,
    Remove,
    Calculate
}

impl Calculator {
    fn new_op(&mut self, op: char) {
        self.equation.op = op.to_string();
        if self.equation.full_equation.len() == 0 {
            match self.input_value.parse::<f64>() {
                Ok(value) => {
                    self.equation.result=value;
                    self.equation.full_equation.push_str(value.to_string().as_str());
                }
                Err(_) => {println!("Error parsing input value");}
            }
        }
        else {
            match self.equation.full_equation.chars().last().unwrap() {
                '+' | '-' | '*' | '/' => {
                    if self.input_value.is_empty() {
                        return;
                    } else {
                        self.calculate();
                    }},
                _ => {println!("{}", self.input_value);}
            }
        }
        self.equation.full_equation.push(op);
        self.input_value = "".to_string();
    }
.

    fn calculate(&mut self) {
        if !self.calculated {
            match self.input_value.parse::<f64>() {
                Ok(value) => {
                    println!("Calculating value: {}", value);
                    println!("Operator: {}", self.equation.op);
                    self.equation.result = match self.equation.full_equation.chars().last().unwrap() {
                        '+' => { self.equation.result + value },
                        '-' => { self.equation.result + -value },
                        '*' => { self.equation.result * value },
                        '/' => { self.equation.result / value },
                        _ => {
                            println!("Unknown op: {}", self.equation.op);
                            0.0
                        },
                    };
                },
                Err(_) => { println!("Something is wrong with the given number {}.", self.input_value); },
            }
            self.calculated = true;
            self.equation.full_equation.push_str(self.input_value.as_str());
        }
        else {
            println!("Already calculated");
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Clear => {
                self.equation = Equation::default();
                self.input_value = "".to_string();
            }
            Message::Add => {
                self.new_op('+')
            }
            Message::Subtract => {
                self.new_op('-')
            }
            Message::Multiply => {
                self.new_op('*')
            }
            Message::Divide => {
                self.new_op('/')
            }
            Message::InputChanged(new_value) => {
                if new_value.is_empty() {
                    self.input_value = "".to_string();
                    return;
                }
                match new_value.chars().last().expect("new_value is {new_value}:") {
                    '.' => {
                        self.input_value = new_value.to_string();
                        self.last_dot = true;
                        return;},
                    ',' => {
                        let new_value = new_value.replace(",", ".");
                        self.input_value = new_value.to_string();
                        self.last_dot = true;
                        return;},
                    '+' | '-' | '*' | '/' => {
                        self.new_op(new_value.chars().last().unwrap());
                        return;
                    },
                    _ => {}
                }
                if self.last_dot & (new_value.chars().last().unwrap() == '0') {
                    self.input_value = new_value.to_string();
                    return;
                }
                println!("Input value: {}", new_value);
                match new_value.parse::<f64>() {
                    Ok(new_value) => {
                        self.input_value = new_value.to_string();
                        self.calculated = false;
                        self.last_dot = false;
                        return;},
                    Err(_) => {println!("The value could not be parsed as a number.");},
                }
            } Message::Remove => {
                self.equation.left.pop();
            }
            Message::Calculate => {
                self.calculate();
            }
            _ => {println!("{:?}", self.input_value);}

        }
    }


    fn view(&self) -> Column<Message> {
        let input = text_input("", &self.input_value)
            .on_input(Message::InputChanged).on_submit(Message::Calculate);
        let button_row = Row::new().spacing(20)
            .push(button("+").on_press(Message::Add))
            .push(button("-").on_press(Message::Subtract))
            .push(button("*").on_press(Message::Multiply))
            .push(button("/").on_press(Message::Divide));
        let settings_row = Row::new().spacing(20)
            .push(button("clear").on_press(Message::Clear))
            .push(button("Remove").on_press(Message::Remove))
            .push(button("calc").on_press(Message::Calculate));
        column![
            text("calculator").width(Fill).align_x(Center),
            text(self.equation.full_equation.clone()).align_x(Center),
            text(self.equation.result.clone()).align_x(Center),
            input,
            button_row,
            settings_row,
            checkbox("Toggle me", self.calculated.clone()),
        ]
    }
}