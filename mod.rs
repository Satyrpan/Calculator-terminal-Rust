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
pub fn evaluate(&mut self, expression: & str) -> Result<f64, Box<dyn Error>> {
    let parsed = Parser::parse(expression)?;
    let result = Operations::calculate(&parsed)?;

    //Adiciona o histórico
    self.history.add_entry(HistoryEntry {
      expression: expression.to_string(),
      result,
      timestamp: chrono::Local::now(),
    });

  Ok(result)
}

///Obtém histórico de operações
pub fn clear_history(&mut self {
    self.history.clear();
