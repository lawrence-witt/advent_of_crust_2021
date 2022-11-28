mod utils;

fn main() {
    let input = common::read_input(10, "\n");
    let mut results: Vec<u64> = Vec::new();
    'lines: for line in input {
        let mut line_result: u64 = 0;
        let mut close_tokens: Vec<utils::Token> = Vec::new();
        for t in line.chars() {
            let token = utils::Token::new(t);
            if token.is_opening() {
                close_tokens.push(token.get_closing().unwrap());
                continue;
            }
            let close_token = close_tokens.pop();
            match close_token {
                Some(t) => if !token.is(t) {
                    continue 'lines
                },
                None => continue 'lines
            }
        }
        for c in close_tokens.iter().rev() {
            line_result *= 5;
            line_result += u64::from(c.score_complete());
        }
        results.push(line_result);
    }
    results.sort();
    println!("{}", results.get(results.len() / 2).unwrap());
}
