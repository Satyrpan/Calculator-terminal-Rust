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
pub fn get_history(&self) -> &Vec<HistoryEntry> {
    self.history.clear();

///Limpa o histórico
pub fn clear_history(&mut self {
    self.history.clear();

///Salva o histórico em um arquivo
pub fn save_history(&self, filename: &str -> Result<(),  Box<dyn Error>> {
    self.history.save_to_file(filename)
}

///Resultado de uma operação
#[derive(debug, clone)]
pub struct OperationResult {
    pub expression: string, 
    pub result: f64
    pub timestamp: chrono::DateTime<chrono::Local>,
  }
  
