use std::env;
use std::io::{self,Read};

use nth::{parse_query,Template};

fn main() {
    let args:Vec<String> = env::args().collect();
    let query=args[1..].join(" ");
    let tpl=parse_query(query);

    let mut lines=String::new();
    io::stdin().read_to_string(&mut lines).unwrap();

    for line in lines.lines()
    {
        let parts:Vec<&str>=line.split_whitespace().collect();
        for item in tpl.iter()
        {
            match item
            {
                &Template::Text(ref t)=>print!("{}",&t),
                &Template::Index(i) if i < parts.len() => print!("{}",&parts[i]),
                _=>(), }
        }
        println!("");
    }
}
