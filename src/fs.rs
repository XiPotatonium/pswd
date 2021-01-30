use crate::cfg::Cfg;
use crate::record::Record;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::str;

/*
pub fn aes_cbc_mode() {
    let message = "Hello World!";

    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];

    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    let encrypted_data = aes256_cbc_encrypt(message.as_bytes(), &key, &iv)
        .unwrap();
    let decrypted_data = aes256_cbc_decrypt(&encrypted_data[..], &key, &iv)
        .unwrap();

    let crypt_message = str::from_utf8(decrypted_data.as_slice()).unwrap();

    assert_eq!(message, crypt_message);
    println!("{}", crypt_message);
}

// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_encrypt(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor =
        aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor
            .encrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();

        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor
            .decrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}
*/

pub fn read_tsv(fname: &str) -> Vec<Record> {
    let fin = BufReader::new(File::open(fname).unwrap());
    let mut ret: Vec<Record> = Vec::new();
    // Skip for title
    for line in fin.lines().skip(1) {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split('\t').collect();
        ret.push(Record {
            site: fields[0].to_owned(),
            username: fields[1].to_owned(),
            password: fields[2].to_owned(),
            note: fields[3].to_owned(),
        })
    }
    ret
}

pub fn save_tsv(fname: &str, records: &Vec<Record>) {
    let mut fout = BufWriter::new(File::create(fname).unwrap());

    // Write title
    fout.write(b"site\tusername\tpassword\tnote\n").unwrap();

    // Write records
    for r in records.iter() {
        writeln!(
            fout,
            "{}\t{}\t{}\t{}",
            r.site, r.username, r.password, r.note
        )
        .unwrap();
    }
}

pub fn load_cfg(fname: &str) -> std::io::Result<Cfg> {
    let mut freader = File::open(fname)?;
    let mut cfg_str = String::new();
    freader.read_to_string(&mut cfg_str).unwrap();
    Ok(serde_json::from_str(&cfg_str).unwrap())
}

pub fn store_cfg(fname: &str, cfg: &Cfg) {
    let mut fwriter = File::create(fname).unwrap();
    let cfg_str = serde_json::to_string(cfg).unwrap();
    fwriter.write(cfg_str.as_bytes()).unwrap();
}

macro_rules! consume_u32_le {
    ($buf: ident, $i: ident) => {
        unsafe {
            let ptr = $buf[$i..$i + std::mem::size_of::<u32>()].as_ptr() as *const u32;
            $i += std::mem::size_of::<u32>();
            u32::from_le(*ptr)
        }
    };
}

macro_rules! consume_str {
    ($buf: ident, $i: ident, $len: ident) => {{
        $i += $len;
        str::from_utf8(&$buf[$i - $len..$i]).unwrap()
    }};
}

pub fn load_records(fname: &str, kword: &String) -> Vec<Record> {
    let path = Path::new(fname);
    if !path.exists() {
        return Vec::new();
    }

    let buf = fs::read(fname).unwrap();
    let mut ret: Vec<Record> = Vec::new();

    let mut i = 0usize;

    while i < buf.len() {
        let site_len = consume_u32_le!(buf, i) as usize;
        let username_len = consume_u32_le!(buf, i) as usize;
        let password_len = consume_u32_le!(buf, i) as usize;
        let note_len = consume_u32_le!(buf, i) as usize;

        let site = String::from(consume_str!(buf, i, site_len));
        let username = String::from(consume_str!(buf, i, username_len));
        let password = String::from(consume_str!(buf, i, password_len));
        let note = String::from(consume_str!(buf, i, note_len));

        ret.push(Record::new(site, username, password, note))
    }

    ret
}

pub fn store_records(fname: &str, records: &Vec<Record>, kword: &String) {
    let mut buf: Vec<u8> = Vec::new();

    for r in records.iter() {
        buf.extend_from_slice(&(r.site.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(r.username.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(r.password.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(r.note.len() as u32).to_le_bytes());

        buf.extend(r.site.as_bytes());
        buf.extend(r.username.as_bytes());
        buf.extend(r.password.as_bytes());
        buf.extend(r.note.as_bytes());
    }

    fs::write(fname, &buf).unwrap();
}
