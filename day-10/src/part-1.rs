mod utils;

fn main() {
    let input = common::read_input(10, "\n");
    let mut result: u32 = 0;
    'lines: for line in input {
        let mut close_tokens: Vec<utils::Token> = Vec::new();
        for t in line.chars() {
            let token = utils::Token::new(t);
            if token.is_opening() {
                close_tokens.push(token.get_closing().unwrap());
                continue;
            }
            let close_token = close_tokens.pop();
            match close_token {
                Some(t) => {
                    if !token.is(t) {
                        result += token.score_illegal();
                        continue 'lines
                    }
                },
                None => {
                    result += token.score_illegal();
                    continue 'lines;
                }
            }
        }
    }
    println!("{result}")
}