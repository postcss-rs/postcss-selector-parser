use plugin_selector_parser::tokenizer;
use std::time;

fn main() {
    let t1 = time::Instant::now();

    let x = " .foo > #foo div ".repeat(1_000_000);

    let input = format!("::v-deep({})", x);
    let mut t = tokenizer::Tokenizer::new(&input, None);
    let tokens = t.tokenize();

    println!("{}s", (t1.elapsed().as_millis() as f32) / 1000f32);

    // for t in tokens.iter() {
    //     print!("{}", t.to_string());
    //     println!("{}", &input[t.pos.0..t.pos.1])
    // }

    drop(tokens)
}
