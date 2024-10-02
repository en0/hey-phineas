mod const_randomizer;

use std::{error::Error, fs::File, io::{self, Result as IoResult, BufRead}};
use argh::FromArgs;

#[derive(FromArgs)]
/// Hey Phineas, what would you like to do today?
struct CommandLineOpts {

    #[argh(option, short = 'f')]
    /// the file with the options
    file_name: String,

    #[argh(option, short = 'r', from_str_fn(randomizer_from_str), default = "Box::new(phineas_machine::BuiltInRandomizer)")]
    /// the randomizer to use
    randomizer: Box<dyn phineas_machine::Randomizer>
}

fn randomizer_from_str(value: &str) -> Result<Box<dyn phineas_machine::Randomizer>, String> {
    let (max_usize, _) = 0usize.overflowing_sub(1);
    match value {
        "built-in" => Ok(Box::new(phineas_machine::BuiltInRandomizer)),
        // Because it is fun, not because it is useful.
        "first" => Ok(Box::new(const_randomizer::ConstantRandomizer::new(0))),
        "last" => Ok(Box::new(const_randomizer::ConstantRandomizer::new(max_usize))),
        _ => Err("Expected one of built-in, first, last".to_string())
    }
}

fn load_options_from_path(file_name: &str) -> IoResult<Vec<String>> {
    let fp = File::open(file_name)?;
    let reader = io::BufReader::new(fp);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn main() -> Result<(), Box<dyn Error>> {
    let default_option = String::from("Make a list I can understand!");
    let opts: CommandLineOpts = argh::from_env();
    let options = load_options_from_path(&opts.file_name)?;
    let limit = options.len();
    let index = opts.randomizer.pick(limit);
    let selected = options.get(index).unwrap_or(&default_option);
    println!("Hey Ferb! I know what we're going to do today!");
    println!("{selected}");
    Ok(())
}
