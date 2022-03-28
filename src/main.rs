use handlebars::Handlebars;
use pulldown_cmark::{html, Options, Parser};
use std::collections::HashMap;

mod node;
mod source;

fn main() {
    let hb_input = "**Hello**, ~~fucking~~ *{{world}}*!";

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("input", hb_input)
        .unwrap();

    let mut data = HashMap::new();
    data.insert("world".to_string(), "世界!".to_string());

    let md_input = handlebars.render("input", &data).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&md_input, options);

    let mut output = String::new();
    html::push_html(&mut output, parser);

    println!("{}", output);
}
