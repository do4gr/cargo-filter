use std::env;
use std::str;
use std::process::Command;
use regex::Regex;
use colored::*;


fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() < 3 {
        println!("{}", "Please provide an argument to filter.".red());
    }else{


        let test_name = &args[2];
        let output = Command::new("cargo")
            .arg("test")
            .arg(test_name)
            .arg("--")
            .arg("--test-threads=1")
            .output()
            .expect("failed to execute process");
    
        let string = str::from_utf8(&output.stdout).unwrap();
        let running_reg = Regex::new(r"running [1-9][1-9]* tests").unwrap();
        let result_reg_success = Regex::new(r"test result: ok. [1-9][0-9]* passed; [0-9]* failed; [0-9]* ignored; [0-9]* measured; [0-9]* filtered out").unwrap();
        let result_reg_failure = Regex::new(r"test result: FAILED. [0-9]* passed; [1-9][0-9]* failed; [0-9]* ignored; [0-9]* measured; [0-9]* filtered out").unwrap();
    
        let running : Vec<usize>= string.lines().enumerate().filter(|(_, v)| running_reg.is_match(v)).map(|(i, _)| i).collect();
        let result : Vec<usize>= string.lines().enumerate().filter(|(_, v)| result_reg_success.is_match(v)|| result_reg_failure.is_match(v)).map(|(i, _)| i).collect();
    
        running.iter().zip(result).for_each(|tup|{
            println!("");
            println!("{}", string.lines().nth(*tup.0).unwrap().yellow().bold());
            string.lines().take(tup.1).skip(*tup.0 + 1 ).for_each(|line| {
                println!("{}", line);
            });
    
            let result = string.lines().nth(tup.1).unwrap();
            if result.contains("test result: ok"){
                println!("{}", result.green());   
            }else{
                println!("{}", result.red());
            }
        })
    }
}