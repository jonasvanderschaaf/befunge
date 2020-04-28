mod parser;

use parser::Parser;

fn main() {
    let mut parser: Parser = Parser::new(Box::new([">v", "^<"]));

    let result = parser.run();
}
