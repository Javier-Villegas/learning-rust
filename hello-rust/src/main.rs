use ferris_says::say;
use std::io::{stdout, BufWriter};





fn push_string(v: &mut Vec<String>, s: String){
    v.push(s);
}


fn main() {
    let stdout = stdout();
    let message = String::from("Hello Rustaceans");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    let mut v = vec![String::from("Sex")];
    let s = String::from("Test");
    push_string(&mut v, s);

    println!("{:?}", v);

}
