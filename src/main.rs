mod cfg;
mod fs;
mod parser;
mod record;

extern crate crypto;
extern crate serde;
extern crate serde_json;

use cfg::Cfg;
use crypto::digest::Digest;
use parser::{argparse, parse, Op};
use record::Record;
use std::env;
use std::io::{self, Write};

macro_rules! prompt {
    ($arg: tt) => {
        print!($arg);
        io::stdout().flush().unwrap();
    };
}

macro_rules! readline {
    ($s: ident) => {
        io::stdin().read_line(&mut $s).unwrap();
        $s = String::from($s.trim_end());
    };
}

/**
 * Set new password from user input
*/
fn get_new_pswd(cfg: &mut Cfg) -> Result<[u8; 32], &'static str> {
    let new_pswd = rpassword::prompt_password_stdout("New password: ").unwrap();
    let repeat_new_pswd = rpassword::prompt_password_stdout("Repeat new password: ").unwrap();
    if new_pswd == repeat_new_pswd {
        let mut hasher = crypto::sha3::Sha3::sha3_256();
        hasher.input_str(&new_pswd);
        let mut pswd1 = [0u8; 32];
        hasher.result(&mut pswd1);
        let mut hasher = crypto::sha3::Sha3::sha3_256();
        hasher.input(&pswd1);
        cfg.pswd = hasher.result_str();
        Ok(pswd1)
    } else {
        Err("Password doesn't match")
    }
}

fn add_record(records: &mut Vec<Record>) {
    loop {
        prompt!("Site: ");
        let mut site = String::new();
        readline!(site);
        if site.is_empty() {
            return;
        }
        let mut username = String::new();
        let mut password = String::new();
        let mut note = String::new();
        prompt!("Username: ");
        readline!(username);
        prompt!("Password: ");
        readline!(password);
        prompt!("Note: ");
        readline!(note);

        records.push(Record::new(site, username, password, note));

        println!();
        record::print_record(records, records.len() - 1);
        println!();
    }
}

fn modify_record(records: &mut Vec<Record>, idx: usize, remain: bool) -> bool {
    prompt!("Site: ");
    let mut site = String::new();
    readline!(site);
    if site.is_empty() && !remain {
        return false;
    }
    let mut username = String::new();
    let mut password = String::new();
    let mut note = String::new();
    prompt!("Username: ");
    readline!(username);
    prompt!("Password: ");
    readline!(password);
    prompt!("Note: ");
    readline!(note);

    // Print record before modification
    record::print_record(records, idx);

    let record = &mut records[idx];
    if remain {
        if !site.is_empty() {
            record.site = site;
        }
        if !username.is_empty() {
            record.username = username;
        }
        if !password.is_empty() {
            record.password = password;
        }
        if !note.is_empty() {
            record.note = note;
        }
    } else {
        record.site = site;
        record.username = username;
        record.password = password;
        record.note = note;
    }

    // Print record after modification
    println!();
    record::print_record(records, idx);

    true
}

static CFG_FNAME: &str = "pswd-cfg.json";
static RECORD_FNAME: &str = "pswd.record";

fn main() {
    // Read and parse console argument
    let mut op = argparse();
    let cfg_path: String;
    let def_record_path: String;
    {
        let mut pbuf = env::current_exe().unwrap();
        pbuf.pop();
        pbuf.push(CFG_FNAME);
        cfg_path = String::from(pbuf.to_str().unwrap());
        pbuf.pop();
        pbuf.push(RECORD_FNAME);
        def_record_path = String::from(pbuf.to_str().unwrap());
    }

    // Load config
    let mut cfg: Cfg;
    let mut pswd1 = [0u8; 32];
    match fs::load_cfg(&cfg_path) {
        Ok(cfg_) => {
            cfg = cfg_;
            // Verify password
            let pswd = rpassword::prompt_password_stdout("Password: ").unwrap();
            // password hashed once, used for encryption
            let mut hasher = crypto::sha3::Sha3::sha3_256();
            hasher.input_str(&pswd);
            hasher.result(&mut pswd1);

            // password hashed twice, used for verification
            let mut hasher = crypto::sha3::Sha3::sha3_256();
            hasher.input(&pswd1);
            let pswd2 = hasher.result_str();
            if pswd2 != cfg.pswd {
                println!("Incorrect password");
                return;
            } else {
                // TODO touch cfg
            }
        }
        _ => {
            println!("{} not exists.", cfg_path);
            cfg = Cfg::new(def_record_path);
            match get_new_pswd(&mut cfg) {
                Ok(pswd1_) => {
                    pswd1 = pswd1_;
                    fs::store_cfg(&cfg_path, &cfg);
                }
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            }
        }
    }

    // Load records from files
    // TODO: use bencrypt to generate key, sha256 to generate iv
    let mut records = fs::load_records(&cfg.record_pth, &pswd1, &pswd1[..16]);
    let mut modified = false;

    // Operations
    loop {
        if op.is_ok() {
            match &op.unwrap() {
                Op::None => {}
                Op::Add => {
                    add_record(&mut records);
                    modified = true;
                }
                Op::Del(idx) => {
                    // Print to-be deleted item
                    record::print_record(&records, *idx);
                    records.remove(*idx);
                    modified = true;
                }
                Op::Ls(clause) => {
                    // list some
                    record::print_table(records.iter().enumerate().filter(|x| clause.check(x.1)));
                }
                Op::Mod(idx, remain) => {
                    if modify_record(&mut records, *idx, *remain) {
                        modified = true;
                    }
                }
                Op::Import(fname) => {
                    prompt!("Warning: import will replace all old records and it cannot be undone!\nStill import?(Y/N):");
                    let mut ans = String::new();
                    readline!(ans);
                    ans = ans.to_uppercase();
                    if ans == "Y" {
                        records = fs::read_tsv(&fname);
                        modified = true;
                    }
                }
                Op::Export(fname) => {
                    fs::save_tsv(&fname, &records);
                }
                Op::ChangePswd => match get_new_pswd(&mut cfg) {
                    Ok(pswd1_) => {
                        pswd1 = pswd1_;
                        modified = true;
                        fs::store_cfg(&cfg_path, &cfg);
                    }
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                },
                Op::Quit => {
                    break;
                }
            }
        } else {
            println!("{}\n", op.unwrap_err());
        }

        prompt!("\n>>> ");
        let mut line = String::new();
        readline!(line);
        // TODO: use regex to match tokens
        op = parse(
            line.split_ascii_whitespace()
                .map(|x| String::from(x))
                .collect(),
        );
    }

    if modified {
        // Save if records is modified
        fs::store_records(&cfg.record_pth, &records, &pswd1, &pswd1[..16]);
    }
}
