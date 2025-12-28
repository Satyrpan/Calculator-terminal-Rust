mod operations;
mod parser; 
mod history;

pub use operations::*;
pub use parser::*;
pub use history::*;

use std:: error::Error;

///Estrutura principal da calculadora
pub struct Calculator {
  history: History,
}

impl Calculator {
  ///Cria uma nova instancia da calculadora
  pub fn new() -> Self {
      Calculator {
          history: History::new (),
      }
  }

///Avalia uma expressão matematica 
