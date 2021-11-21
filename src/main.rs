use plugin_selector_parser::tokenizer;

fn main() {
    let input = "::v-deep( .foo > #foo div )";
    let mut t = tokenizer::Tokenizer::new(input, None);
    let tokens = t.tokenize();

    for t in tokens.iter() {
        print!("{}", t.to_string());
        println!("{}", &input[t.pos.0..t.pos.1])
    }

    drop(tokens)
}
