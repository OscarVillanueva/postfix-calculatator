mod post_notation;

use post_notation::postfix;

fn main() {

    println!("Ingresa una operación matemática");

    let mut notation = String::new();
    std::io::stdin().read_line(&mut notation).unwrap();
    notation = notation.trim().to_string();

    postfix::convert_to_post_fix(&notation);

}
