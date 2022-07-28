
pub mod postfix {
  use std::char;

  pub struct Supervisor {
    pub postfix: String,
    pub stack: String
  }
  
  impl Supervisor {
    pub fn new() -> Supervisor{
      Supervisor { 
        postfix: "".to_string(), 
        stack: "".to_string()
      }
    }
  }

  fn get_symbol_priority(symbol: &char) -> u8 {
      match symbol {
        &'(' | ')' => 4,
        &'^' => 3,
        &'*' => 2,
        &'/' => 2,
        &'+' => 1,
        &'-' => 1,
        _ => 0
      }
  }

  // e -> stack empty
  // n -> no hay operador que se vaya a agregar
  fn add_to_stack(stack: &String, operator: char, priority: u8) -> (String, String) {
    
    let mut taken_out = "n".to_string();
    let mut new_stack = stack.clone();

    let mut chars = stack.chars();
    let mut stack_top = chars.next().unwrap_or('e');
    let mut previos_priority = get_symbol_priority(&stack_top);

    // Si la pila esta vacía mete el nuevo símbolo
    if stack_top == 'e' {
      new_stack.push(operator);
      return (taken_out, new_stack)
    }

    // Si es un ) limpia la pila
    if priority == 4  {
      if operator == ')' {

        taken_out = stack_top.to_string();
  
        loop {
  
          let dropped = chars.next().unwrap_or('e');
          if dropped == '(' || dropped == 'e' { break; }
  
        }

        new_stack = chars.collect();
        return (taken_out, new_stack)
      }

      new_stack = operator.to_string();
      new_stack.push(stack_top);
      new_stack.push_str(chars.as_str());
      return (taken_out, new_stack);

    }

    if previos_priority >= priority && stack_top != '(' {

      taken_out = stack_top.to_string();
      taken_out.push(' ');

      loop {
        
        stack_top = chars.next().unwrap_or('e');
        previos_priority = get_symbol_priority(&stack_top);

        if stack_top == 'e'{

          new_stack = operator.to_string();
          return (taken_out, new_stack)

        }

        if priority != previos_priority {

          new_stack = operator.to_string();
          new_stack.push(stack_top);
          new_stack.push_str(chars.as_str());
          return (taken_out, new_stack)

        }

        taken_out.push(stack_top);

      }

    }

    new_stack = operator.to_string();
    new_stack.push(stack_top);
    new_stack.push_str(chars.as_str());
    return (taken_out, new_stack);

  }

  pub fn convert_to_post_fix(notation: &String ) -> String{

    let mut supervisor = Supervisor::new();
    let mut priority: u8;

    for symbol in notation.chars() {

      priority = get_symbol_priority(&symbol);

      if [' ', '\n', '\r', '\t'].contains(&symbol) { continue; }

      if symbol == ';' {
        supervisor.postfix.push(' ');
        supervisor.postfix.push_str(&supervisor.stack);
        supervisor.postfix.push(symbol);

        if supervisor.postfix.contains("(") || supervisor.postfix.contains(")") {
          panic!("Operación invalida");
        }

      }
      else {
        if priority == 0 {
          supervisor.postfix.push(symbol)
        }
        else {

          if symbol != '(' {
            supervisor.postfix.push(' ');
          }

          let ( taken_out, new_stack ) = add_to_stack(&supervisor.stack, symbol, priority);
          supervisor.stack = new_stack;

          if taken_out != "n" {
            supervisor.postfix.push_str(&taken_out);
            supervisor.postfix.push(' ');
          }
        }
      }

    }

    supervisor.postfix
    
  }
}