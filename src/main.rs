mod post_notation;

use post_notation::postfix;

fn main() {

    // println!("Ingresa una operación matemática");

    // let mut notation = String::new();
    // std::io::stdin().read_line(&mut notation).unwrap();
    // notation = notation.trim().to_string();
    let mut post: String;
    let notations = [
        "(AX+(B*C))", 
        "((AX+(B*CY))/(D-E))",
        "((A+B)*(C+E))",
        "(AX*(BX*(((CY+AY)+BY)*CX)))",
        "((H*((((A+((B+C)*D))*F)*G)*E))+J)",
        "(A+B/C*(D+E)-F)"
    ];

    for notation in notations {

        post = postfix::convert_to_post_fix(&notation.to_string());
        println!("infix: {} - postfix: {}", notation, post);
        println!("")


    }

}
