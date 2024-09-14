use rstest::rstest;
use anyhow::{Result,anyhow};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("'")]
    StartLineComment,

    #[token("\n", priority = 3)]
    EOL,

    #[token("[[")]
    StartBlock,

    #[token("]]")]
    EndBlock,

    #[regex(r"FILES")]
    FilesKeyword,

    #[regex(r"match")]
    MatchKeyword,

    #[regex(r"=")]
    Assignment,

    #[regex(r"(\./|/)[a-zA-Z0-9_]+\.[a-z]+")]
    FileName,

    #[token(";")]
    Semicolon,

    #[regex(r"[ \t\n\r]+")]
    Whitespace,

    #[regex(r"[a-zA-Z0-9_]+\.[a-z]+")]
    Litteral,
}

#[rstest]
#[case("' [[ FILES = ./bowser.puml ./wario.puml ./waluigi.puml;]]\n")]
#[case("' [[ match FILES\n")]
#[case("' ]]\n")]
fn test_individual_parsing(#[case] case: &str) -> Result<()> {
    let mut lexer = Token::lexer(case);

    while let Some(token) = lexer.next() {
        match token {
            Ok(_) => println!("{:?}: {:?}", token, lexer.slice()),
            _ => return Err(anyhow!("Could not parse {:?} in {:?}", lexer.slice(), &case)),
        }
    }
    Ok(())
}
