pub struct Record {
    pub site: String,
    pub username: String,
    pub password: String,
    pub note: String,
}

impl Record {
    pub fn new(site: String, username: String, password: String, note: String) -> Record {
        Record {
            site,
            username,
            password,
            note,
        }
    }
}

pub fn print_table(records: &Vec<Record>) {
    println!("\tSite\tUsername\tPassword\tNote\n");
    for (i, r) in records.iter().enumerate() {
        println!(
            "{}\t{}\t{}\t{}\t{}",
            i, r.site, r.username, r.password, r.note
        );
    }
}

pub fn print_record(records: &Vec<Record>, idx: usize) {
    let r = &records[idx];
    println!("\tSite\tUsername\tPassword\tNote");
    println!(
        "{}\t{}\t{}\t{}\t{}",
        idx, r.site, r.username, r.password, r.note
    );
}