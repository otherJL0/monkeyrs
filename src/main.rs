mod ast;
mod lexer;
mod parser;
mod token;
use crate::parser::Parser;
use reedline::{
    DefaultPrompt, DefaultPromptSegment, ExampleHighlighter, Reedline, Signal, Vi,
    default_vi_insert_keybindings, default_vi_normal_keybindings,
};

fn repl() {
    let commands = vec![
        "let".into(),
        "if".into(),
        "else".into(),
        "fn".into(),
        "true".into(),
        "false".into(),
    ];
    let mut line_editor = Reedline::create()
        .with_edit_mode(Box::new(Vi::new(
            default_vi_insert_keybindings(),
            default_vi_normal_keybindings(),
        )))
        .with_highlighter(Box::new(ExampleHighlighter::new(commands)));
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("monkey".to_string()),
        DefaultPromptSegment::Empty,
    );
    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let mut parser = Parser::new(&buffer);
                if let Some(program) = parser.parse_program() {
                    println!("{program}");
                } else {
                    println!("Invalid statement");
                    for error in parser.errors {
                        print!("{error}");
                    }
                }
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("Aborted");
                break;
            }
            x => {
                println!("{x:?}");
            }
        }
    }
}

fn main() {
    repl();
}
