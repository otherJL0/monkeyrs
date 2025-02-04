mod lexer;
mod token;
use crate::lexer::Lexer;
use reedline::{
    DefaultPrompt, DefaultPromptSegment, ExampleHighlighter, Reedline, Signal, Vi,
    default_vi_insert_keybindings, default_vi_normal_keybindings,
};
use token::TokenType;

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
                let mut lexer = Lexer::new(&buffer);
                let mut token = lexer.next_token();
                while token.token_type != TokenType::Eof {
                    println!("{token:?}");
                    token = lexer.next_token();
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
