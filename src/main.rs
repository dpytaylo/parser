use parser::{self, Parser};

// fn main() {
//     let parser = Parser::from("code endword");
//     let _word = parser.get_to_chars(&['d', 'o', 'w']);

//     parser.get_word();
// }

fn main() {
    let parser = Parser::from("абеёжэюя");
    parser.get_to_str("её");

    assert_eq!(parser.next(), 'е');
}