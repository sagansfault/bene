use std::{error::Error, str::FromStr};

use regex::Regex;

pub struct Intake<'a> {
    args: Vec<Arg<'a>>
}

impl<'a> Intake<'a> {
    pub fn new() -> Intake<'a> {
        Intake { args: vec![] }
    }

    /// Adds an argument to the intake to which the intake will try to parse and mutate
    /// 
    /// ### Arguments
    /// * 'cmd_short' - a short version of the command flag, one character
    /// * 'cmd_long' - a long verion of the command flag, one word
    /// 
    /// ### Example
    /// ```rust
    /// bene::Intake::new()
    ///     .arg('f', "frames", &mut frames)
    ///     .arg('L', "lib", &mut lib);
    /// ```
    /// 
    pub fn arg(mut self, cmd_short: char, cmd_long: &str, arg: &'a mut dyn ArgValue) -> Intake<'a> {
        let formatted = format!(r"(?P<short>-{} \w*)|(?P<long>--{} \w*)", cmd_short, cmd_long);
        let regex = Regex::new(formatted.as_str()).unwrap();
        self.args.push(Arg { arg, regex });
        self
    }

    /// Runs the intake on a given input, this will process via regex and mutate found values
    pub fn process(&mut self, input: &str) {
        let input = format!("{} ", input.trim());
        for arg in self.args.iter_mut() {
            for caps in arg.regex.captures_iter(&input) {
                if let Some(short) = caps.name("short") {
                    if let Some((_, mut val)) = short.as_str().split_once(" ") {
                        if val.is_empty() {
                            val = "true";
                        }
                        let _ = arg.arg.set_value(val);
                    }
                }

                if let Some(long) = caps.name("long") {
                    if let Some((_, mut val)) = long.as_str().split_once(" ") {
                        if val.is_empty() {
                            val = "true";
                        }
                        let _ = arg.arg.set_value(val);
                    }
                }
            }
        }
    }
}

pub struct Arg<'a> {
    arg: &'a mut dyn ArgValue,
    regex: Regex
}

pub trait ArgValue {
    fn set_value(&mut self, input: &str) -> Result<(), Box<dyn Error>>;
}

impl<T> ArgValue for T where T: FromStr, <T as FromStr>::Err: std::fmt::Debug + Error + 'static {
    fn set_value(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let val = input.parse::<T>()?;
        *self = val;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_test() {
        let input = "-f 20 -L --length 5 --lib";

        let mut frames: usize = 30;
        let mut length = 3;
        let mut lib = false;

        Intake::new()
            .arg('f', "frames", &mut frames)
            .arg('l', "length", &mut length)
            .arg('L', "lib", &mut lib)
            .process(input);

        assert_eq!(frames, 20);
        assert_eq!(length, 5);
        assert_eq!(lib, true);
    }
}
