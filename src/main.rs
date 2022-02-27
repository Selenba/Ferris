use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::env;

fn vec_to_string(input: &Vec<String>) -> String {
    let mut result: String = "".to_string();

    for s in input{
	result.push_str(&s);
	result.push_str(&" ".to_string())
    }

    return result;
}

fn main() {
    let mut args: Vec<String> = env::args().collect();	
    args.remove(0);    
    let stdout = stdout();
    let message: String = vec_to_string(&args);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
