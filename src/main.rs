use std::env;
use unicode_segmentation::UnicodeSegmentation;
use std::io::{self,Read};

#[derive(Debug)]
enum Template
{
    Text(String),
    Index(usize),
}

enum Current
{
    NothingYet,
    Number(usize),
    Text(String),
    Escaped,
    Quoted(String),
}

fn parse_query(query:String)->Vec<Template>
{
    // each chars can contains a string or a num
    let mut curr:Current=Current::NothingYet;

    // will store text and column index
    let mut tpl:Vec<Template>=Vec::new();

    let list=query.graphemes(true);
    for graph in list
    {
        let digit=graph.parse::<usize>();
        match digit
        {
            // Found a digit
            Ok(d)=>
            {
                curr=match curr
                {
                    Current::NothingYet=>Current::Number(d),
                    Current::Number(n)=>Current::Number(n*10+d),
                    Current::Text(ref t)=>
                    {
                        tpl.push(Template::Text(t.to_string()));
                        Current::Number(d)
                    }
                    Current::Escaped=>Current::Text(String::from(graph)),
                    Current::Quoted(ref mut t)=>
                    {
                        t.push_str(&String::from(graph)[..]);
                        Current::Quoted(t.to_string())
                    }
                }
            },

            // Found something else
            Err(_)=>
            {
                // Want to escape the next char
                if graph=="\\"
                {
                    match curr
                    {
                        Current::Number(n)=>tpl.push(Template::Index(n)),
                        Current::Text(ref t)=>tpl.push(Template::Text(t.to_string())),
                        Current::Quoted(ref mut t)=>
                        {
                            t.push_str("\\");
                            tpl.push(Template::Text(t.to_string()));
                        }
                        _ => (),
                    }
                    curr=Current::Escaped;
                    continue
                }
                if graph=="\""
                {
                    match curr
                    {
                        Current::Number(n)=>tpl.push(Template::Index(n)),
                        Current::Text(ref t)=>tpl.push(Template::Text(t.to_string())),
                        Current::Quoted(ref t)=>tpl.push(Template::Text(t.to_string())),
                        Current::Escaped=>tpl.push(Template::Text("\"".to_string())),
                        _ => (),
                    }
                    println!("Quoted");
                    curr=Current::Quoted(String::new());
                    continue
                }
                curr=match curr
                {
                    Current::NothingYet=>Current::Text(String::from(graph)),
                    Current::Number(n)=>
                    {
                        tpl.push(Template::Index(n));
                        Current::Text(String::from(graph))
                    }
                    Current::Text(ref mut t)=>
                    {
                        t.push_str(&String::from(graph)[..]);
                        Current::Text(t.to_string())
                    }
                    Current::Escaped=>
                    {
                        match graph
                        {
                            "t"=>Current::Text(String::from("\t")),
                            "n"=>Current::Text(String::from("\n")),
                            "\\"=>Current::Text(String::from("\\")),
                            _=>Current::Text(String::from(graph)),
                        }
                    },
                    Current::Quoted(ref mut t)=>
                    {
                        t.push_str(&String::from(graph)[..]);
                        Current::Quoted(t.to_string())
                    }
                }
            },
        };
    }
    match curr
    {
        Current::Number(n)=>tpl.push(Template::Index(n)),
        Current::Text(ref t)=>tpl.push(Template::Text(t.to_string())),
        Current::Quoted(ref t)=>tpl.push(Template::Text(t.to_string())),
        _=>(),
    }
    tpl
}

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
