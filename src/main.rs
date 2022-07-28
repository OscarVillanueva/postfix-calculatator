mod post_notation;
mod calculator;

use post_notation::postfix;
use calculator::solve_postfix;

fn main() {

    loop {

        println!("Ingresa una operación matemática terminando con ;, para salir q");

        let mut notation = String::new();
        std::io::stdin().read_line(&mut notation).unwrap();
        notation = notation.trim().to_string();

        if notation == "q" { break; }
    
        if !notation.ends_with(";") { panic!("La operación debe terminar con ;"); }
    
        let mut post = postfix::convert_to_post_fix(&notation);
        post = post.replace("  ", " ");

        let result = solve_postfix::resolve_postfix(&post);
    
        println!("{} = {}", notation, result);
    }

}
