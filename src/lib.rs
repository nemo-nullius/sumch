extern crate regex;
use regex::Regex;

use std::io;
use std::fs::File;
use std::io::prelude::*;

struct Reholder{
    mean: &'static str,
    re: &'static str,
}

impl Reholder {
    fn new() -> Vec<Reholder> {
        vec![
            Reholder {
                mean: "all",
                re: r".",
            },
            Reholder {
                mean: "en",
                re: r"[a-zA-Z0-9_]+",
            },
            Reholder {
                mean: "cn",
                re: r"[\u{4E00}-\u{9FA5}\u{9FA6}-\u{9FEF}\u{3400}-\u{4DB5}\u{20000}-\u{2A6D6}\u{2A700}-\u{2B734}\u{2B740}-\u{2B81D}\u{2B820}-\u{2CEA1}\u{2CEB0}-\u{2EBE0}\u{2F00}-\u{2FD5}\u{2E80}-\u{2EF3}\u{F900}-\u{FAD9}\u{2F800}-\u{2FA1D}\u{E815}-\u{E86F}\u{E400}-\u{E5E8}\u{E600}-\u{E6CF}\u{31C0}-\u{31E3}\u{2FF0}-\u{2FFB}\u{3105}-\u{312F}\u{31A0}-\u{31BA}\u{3007}]",
        //r"[\u4E00-\u9FA5\u9FA6-\u9FEF\u3400-\u4DB5\u{20000}-\u{2A6D6}\u{2A700}-\u{2B734}\u{2B740}-\u{2B81D}\u{2B820}-\u{2CEA1}\u{2CEB0}-\u{2EBE0}\u2F00-\u2FD5\u2E80-\u2EF3\uF900-\uFAD9\u{2F800}-\u{2FA1D}\uE815-\uE86F\uE400-\uE5E8\uE600-\uE6CF\u31C0-\u31E3\u2FF0-\u2FFB\u3105-\u312F\u31A0-\u31BA\u3007]"];
            },
            ]
    }
}

pub fn get_from_file(filename: &str) -> Result<String, io::Error>{
    //let mut f = match File::open(filename) {
        //Ok(file) => file,
        //Err(e) => return Err(e),
    //};
    //let mut f = File::open(filename)?;
    //let mut s = String::new();
    //f.read_to_string(&mut s)?;
    //Ok(s)
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn get_from_stdin() -> Result<String, io::Error>{
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn sum_com(s: &str, reholder_arr: &[Reholder]) -> Vec<usize> {
    let mut result = Vec::new();
    let re_iter = reholder_arr.iter().map(|x| x.re);
    for re_str in re_iter {
        let re = Regex::new(re_str).unwrap();
        let all = re.find_iter(s);
        result.push(all.count());
    }
    result
}

pub fn run(s: &str) {
    let reholder = Reholder::new();
    let sum = sum_com(s, &reholder);
    sum.iter()
       .zip(reholder.iter())
       .for_each(|(x,y)| println!("{}: {}", y.mean, x));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result_mix() {
        let content = "Good Good Study,
Day Day Up!
好好学习，天天向上！
𪱈𣛧";

        assert_eq!(vec![39, 6, 10], sum_com(content, &Reholder::new()));
    }
}
