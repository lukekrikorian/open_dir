extern crate colored;
extern crate urlencoding;
extern crate webbrowser;

use colored::*;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use urlencoding::encode;

struct Query<'a> {
    search_term: String,
    include: Vec<&'a str>,
    exclude: &'a str,
}

impl<'a> Query<'a> {
    fn generate_query(&self) -> String {
        let query = format!(
            "intext:\"{}\" intitle:\"index.of\" +({}) {}",
            self.search_term,
            self.include.join("|"),
            self.exclude
        );
        return format!(
            "https://www.google.com/search?q={}",
            encode(&query).replace("%20", "+")
        );
    }
}

fn get_input(query: &str, test: &Fn(&String) -> bool) -> String {
    loop {
        let mut resp = String::new();
        print!("<{}> ", query.blue());
        
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut resp).unwrap();
        resp = resp[..resp.len() - 1].to_string();
        
        if !test(&resp) { continue; }
        return resp;
    }
}

fn main() {
    let exclude = "-inurl:(jsp|pl|php|html|aspx|htm|cf|shtml)";
    let file_extensions: HashMap<String, Vec<&str>> = [
        (
            "video".to_string(),
            vec!["wmv", "mpg", "avi", "mp4", "mkv", "mov"],
        ),
        (
            "audio".to_string(),
            vec!["ac3", "flac", "m4a", "mp3", "ogg", "wav", "wma"],
        ),
        (
            "image".to_string(),
            vec!["bmp", "gif", "jpg", "png", "psd", "tif", "tiff"],
        ),
    ].iter().cloned().collect();

    println!("{}", "filetypes: audio, video, image\n".blue());
    
    let filetype = get_input("filetype", &|s| file_extensions.contains_key(s));
    let search_term = get_input("search term", &|s| s.len() > 0);

    let query = Query {
        search_term,
        include: file_extensions.get(&filetype).unwrap().to_vec(),
        exclude,
    }.generate_query();
    if !webbrowser::open(&query).is_ok() {
        println!("{}", query);
    }
}
