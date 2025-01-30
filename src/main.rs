
use iced::widget::{button, column, text, Column, text_input};
use iced::{Center, Element, Fill, Font, Subscription, Task as Command};
use iced::widget::shader::wgpu::naga::Expression;
use iced::widget::shader::wgpu::naga::MathFunction::Exp;
use regex::Regex;

pub fn main() { // -> iced::Result {
    // iced::run("A cool counter", Counter::update, Counter::view)
    let equation = vec![String::from("3.3"),
                        String::from("+"),
                        String::from("3.3")];
    let mut expr = Expr::BinaryOp {
        op: '+',
        left: Box::new(Expr::Number(3.3)),
        right: Box::new(Expr::Number(3.3)),
    };
    println!("{:?}", equation);
    println!("expr: {:?}", expr.evaluate());
}

fn is_float(s: String) -> bool {
    s.parse::<f64>().is_ok()
}
//TODO: create a function, that takes a vec and creates a Expr
fn create_ast(equation: &mut Vec<String>) -> Expr {
    if !is_float(equation[0].to_string()) {
        equation.insert(0, String::from("0.0"));
    }
    let mut last_float: bool = false;
    for part in equation.iter() {
        if is_float(part.to_string()) & last_float {
            panic!("Two number in a row, no calculation possible.")
        }
    };
}

#[derive(Default)]
struct Counter {
    equation: Vec<String>,
    input_value: String,
    number: String,
    result: String,
}

#[derive(Debug)]
enum Expr {
    Number(f64),
    BinaryOp {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    fn evaluate(&mut self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::BinaryOp { op, left, right } => {
                let left_val = left.evaluate();
                let right_val = right.evaluate();
                match op {
                    '+' => left_val + right_val,
                    '-' => left_val - right_val,
                    '*' => left_val * right_val,
                    '/' => left_val / right_val,
                    _ => panic!("Unknown operator"),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Add,
    Subtract,
    Clear,
    Remove,
    Calculate
}

impl Counter {
    fn add(&mut self) {
        self.equation.push(self.number.clone());
        self.equation.push("+".to_string())
    }

    fn subtract(&mut self) {
        self.equation.push(self.number.clone());
        self.equation.push("-".to_string())
    }

    fn create_calc_tree(&mut self) {

    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Clear => {
                self.equation = Vec::new();
            }
            Message::Add => {
                self.add()
            }
            Message::Subtract => {
                self.subtract()
            }
            Message::InputChanged(new_value) => {
                match new_value.parse::<u8>() {
                    Ok(new_value) => {self.number.push_str(new_value.to_string().as_str());
                    return;},
                    Err(_) => {println!("The value could not be parsed as a number.");},
                }
                if new_value.trim() == '+'.to_string() {
                    self.add()
                } else if new_value.trim() == " - ".to_string() {
                    self.subtract()
                } else if new_value.trim() == '.'.to_string() {
                    self.number.push_str(".");
                }
            } Message::Remove => {
                self.equation.pop();
            }
            Message::Calculate => {
                self.create_calc_tree()
            }

        }
    }

    fn view(&self) -> Column<Message> {
        let input = text_input(" ", &*self.input_value)
            .on_input(Message::InputChanged).padding(5);
        column![
            text("calculator").width(Fill).align_x(Center),
            text(self.equation.join("")).align_x(Center),
            text(self.result.clone()).align_x(Center),
            input,
            button("+").on_press(Message::Add),
            button("-").on_press(Message::Subtract),
            button("clear").on_press(Message::Clear),
            button("remove").on_press(Message::Remove),
            button("calc").on_press(Message::Calculate),
        ]
    }
}