pub mod solve_postfix {

  fn get_result(a: f32, b: f32, operator: char) -> f32{
    match operator {
      '^' => a.powf(b),
      '*' => a * b,
      '/' => a / b,
      '+' => a + b,
      '-' => a - b,
      _ => panic!("Operador invalido")
    }
  }

  pub fn resolve_postfix(notation: &String) -> f32 {
    
    let mut tail_parsed: Result<f32, std::num::ParseFloatError>;
    let mut stack: Vec<f32> = Vec::new();
    let mut tail: String = String::new();

    for char in notation.chars() {

      if char == ';' { continue; }

      if char == ' ' { 

        if tail == "" { continue; }

        tail_parsed = tail.parse::<f32>();

        match tail_parsed {
          Ok(_) => stack.push(tail_parsed.unwrap()),
          Err(_) => panic!("Carácter invalido: {}", tail)
        }

        tail = "".to_string();
        continue; 
      }

      if char >= '0' && char <= '9' {
        tail.push(char);
      }
      else {
        
        if stack.is_empty() { 
          panic!("No se pudo resolver tu operación");
        }

        if let Some(b) = stack.pop() {
          if let Some(a) = stack.pop() {
            stack.push( get_result(a, b, char) )
          }
          else { panic!("No se pudo resolver tu operación") }
        } else { panic!("No se pudo resolver tu operación") }

      }

    }

    if let Some(result) = stack.pop() {
      return result;
    }

    panic!("No se pudo resolver tu operación");

  }

}