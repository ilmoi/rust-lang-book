use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    //notice how all the code that does error printing is in one single function

    // let args:Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // FAKE ARGS
    let args: Vec<String> = vec!["executable".into(), "who".into(), "src/poem.txt".into()];

    //we use unwrap_or_else here because we do need the return value
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing args, full err: {}", err); //we redirect errors to stderr instead of stdout
        process::exit(1);
    });
    // println!("{:?}", config);

    //we use if let here coz we don't need the Ok() return value
    if let Err(e) = run(config) {
        eprintln!("problem when running app, full err: {}", e); //we redirect errors to stderr instead of stdout
        process::exit(1);
    }
}

#[derive(Debug)]
struct Config {
    query: String,
    file: String,
    is_sensitive: bool,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        //old
        // if args.len() < 3 {
        //     return Err("too few args".into());
        // }
        // let query = args[1].clone();
        // let file = args[2].clone();


        //new
        let mut args_iter = args.iter();
        args_iter.next(); //skip the 0th
        // let query = args_iter.next().expect("no query");
        // let file = args_iter.next().expect("no file");

        let query = match args_iter.next() {
            Some(arg) => arg.clone(), //todo have to clone here because why? https://stackoverflow.com/questions/67397018/move-occurs-because-arg-has-type-string-which-does-not-implement-the-copy
            None => return Err("no query"),
        };

        let file = match args_iter.next() {
            Some(arg) => arg.clone(),
            None => return Err("no file"),
        };

        let is_sensitive = env::var("NOT_SENS").is_err();

        Ok(Config { query, file, is_sensitive })
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    // println!("{}", contents);

    let v;

    if config.is_sensitive {
        v = search(&config.query, &contents);
    } else {
        v = search_insensitive(&config.query, &contents);
    }

    println!("final vector that was found is: {:?}", v);

    Ok(())
}


fn search<'a>(query:&str, contents:&'a str) -> Vec<&'a str> {
    //old
    // let mut v: Vec<&str> = vec![];
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         v.push(line);
    //     }
    // }
    // v

    //new
    contents
        .lines()
        .filter(|x| x.contains(query))
        .collect()
}


fn search_insensitive<'a>(query:&str, contents:&'a str) -> Vec<&'a str> {
    // let mut v: Vec<&str> = vec![];
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query.to_lowercase()) {
    //         v.push(line);
    //     }
    // }
    // v

    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "word";
        let content = "\
word exists
does not exist";
        assert_eq!(search(&query, &content), vec!["word exists"])
    }

    #[test]
    fn test_search_insensitive() {
        let query = "wOrD";
        let content = "\
word exists
does not exist";
        assert_eq!(search_insensitive(&query, &content), vec!["word exists"])
    }


}