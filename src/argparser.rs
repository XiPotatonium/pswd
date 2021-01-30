use std::env;

pub enum WhereClause {
    Keyword(String),
    Clause(),
}

pub enum Op {
    Add,
    Del(usize),
    Ls(Option<Box<WhereClause>>),
    Mod(usize, bool),
    Import(String),
    Export(String),
    ChangePswd,
}

pub fn argparse() -> Op {
    let mut args = env::args();

    if args.len() <= 1 {
        panic!("No enough arguments. See usage.");
    }

    let _prog_name = args.next().unwrap();
    let op = args.next().unwrap();

    let op_args: Vec<String> = args.collect();

    match op.as_str() {
        "add" => {
            assert_eq!(op_args.len(), 0, "Invalid usage. See usage.");
            Op::Add
        }
        "del" => {
            assert_eq!(op_args.len(), 1, "Invalid usage. See usage.");
            Op::Del(op_args[0].parse::<usize>().unwrap())
        }
        "ls" => {
            if op_args.len() == 0 {
                // Select all
                Op::Ls(Option::None)
            } else {
                unimplemented!("advanced ls not implemented");
            }
        }
        "mod" => {
            if op_args.len() == 0 {
                // default modify
                Op::Mod(op_args[0].parse::<usize>().unwrap(), false)
            } else if op_args.len() == 1 && op_args[0] == "-e" {
                // Empty input means no change
                Op::Mod(op_args[0].parse::<usize>().unwrap(), true)
            } else {
                panic!("Invalid usage. See usage.");
            }
        }
        "import" => {
            assert_eq!(op_args.len(), 1, "Invalid usage. See usage.");
            Op::Import(String::from(&op_args[0]))
        }
        "export" => {
            assert_eq!(op_args.len(), 1, "Invalid usage. See usage.");
            Op::Export(String::from(&op_args[0]))
        }
        "chpswd" => {
            assert_eq!(op_args.len(), 0, "Invalid usage. See usage.");
            Op::ChangePswd
        }
        _ => {
            panic!(format!("Unsupported command {}", op));
        }
    }
}
