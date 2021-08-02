use parser::{self, Parser};

#[test]
fn parser_test() {
    let parser = Parser::from("a   word getto[ endlol");

    assert_eq!(parser.get_char(), 'a');

    parser.next();
    parser.skip();

    assert_eq!(parser.get_word(), "word");

    parser.next();
    assert_eq!(parser.get_to_chars(&[' ', '[']), "getto");

    parser.next();
    parser.skip();

    assert_eq!(parser.get_to_str("lol"), "end");

    let parser = Parser::from("odfosdf _)_data");
    parser.skip_to_str("_)_");

    assert_eq!(parser.next(), '_');
    parser.next();
    parser.next();

    assert_eq!(parser.get_to_end(), "data");
}