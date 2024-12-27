use std::io::Error;

pub mod config;

pub fn search<I, F>(query: &str, lines: I, mut f: F) -> ()
where
    I: Iterator<Item = Result<String, Error>>,
    F: FnMut(&str) -> (),
{
    for line in lines {
        match line {
            Ok(s) => {
                if s.contains(query) {
                    f(&s);
                }
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_find_a_single_result() {
        let parts = "\
This"
            .split("\n")
            .map(|s| Ok(s.to_owned()));
        let mut results: Vec<String> = Vec::new();
        let pusher = |line: &str| results.push(line.to_owned());
        search("This", parts, pusher);
        assert_eq!(results, vec!["This"]);
    }
}
