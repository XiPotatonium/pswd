use crate::record::Record;
use std::env;

#[derive(std::fmt::Debug)]
pub enum WhereClause {
    Keyword(String),
    Clause(),
    None,
}

impl WhereClause {
    pub fn check(&self, record: &Record) -> bool {
        match self {
            WhereClause::Keyword(s) => {
                record.site.contains(s)
                    || record.username.contains(s)
                    || record.password.contains(s)
                    || record.note.contains(s)
            }
            WhereClause::None => true,
            _ => unimplemented!("Advanced ls not implemented"),
        }
    }
}

#[derive(std::fmt::Debug)]
pub enum Op {
    Add,
    Del(usize),
    Ls(WhereClause),
    Mod(usize, bool),
    Import(String),
    Export(String),
    ChangePswd,
    Quit,
    None,
}

static TOO_FEW_ARGS_MSG: &str = "Too few arguments";
static TOO_MANY_ARGS_MSG: &str = "Too many arguments. See usage.";

pub fn parse(args: Vec<String>) -> Result<Op, &'static str> {
    if args.len() == 0 {
        return Ok(Op::None);
    }

    match args[0].as_str() {
        "add" => match args.len() {
            1 => Ok(Op::Add),
            _ => Err(TOO_MANY_ARGS_MSG),
        },
        "del" => match args.len() {
            1 => Err(TOO_FEW_ARGS_MSG),
            2 => Ok(Op::Del(args[1].parse::<usize>().unwrap())),
            _ => Err(TOO_MANY_ARGS_MSG),
        },
        "ls" => match args.len() {
            1 => Ok(Op::Ls(WhereClause::None)),
            2 => Ok(Op::Ls(WhereClause::Keyword(args[1].clone()))),
            _ => unimplemented!("Advanced ls not implemented"),
        },
        "mod" => match args.len() {
            1 => Err(TOO_FEW_ARGS_MSG),
            2 => Ok(Op::Mod(args[1].parse::<usize>().unwrap(), false)),
            3 if args[2] == "-e" => Ok(Op::Mod(args[1].parse::<usize>().unwrap(), true)),
            _ => Err(TOO_MANY_ARGS_MSG),
        },
        "import" => match args.len() {
            1 => Err(TOO_FEW_ARGS_MSG),
            2 => Ok(Op::Import(args[1].clone())),
            _ => Err(TOO_MANY_ARGS_MSG),
        },
        "export" => match args.len() {
            1 => Err(TOO_FEW_ARGS_MSG),
            2 => Ok(Op::Export(args[1].clone())),
            _ => Err(TOO_MANY_ARGS_MSG),
        },
        "chpswd" => match args.len() {
            1 => Ok(Op::ChangePswd),
            _ => Err(TOO_MANY_ARGS_MSG),
        },
        "q" => match args.len() {
            1 => Ok(Op::Quit),
            _ => Err(TOO_MANY_ARGS_MSG),
        },
        _ => Err("Unknown command"),
    }
}

pub fn argparse() -> Result<Op, &'static str> {
    let mut args = env::args();

    let _prog_name = args.next().unwrap();

    parse(args.collect())
}
