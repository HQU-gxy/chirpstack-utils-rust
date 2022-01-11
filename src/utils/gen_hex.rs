use rand::Rng;

fn get_rand_half_octet() -> u8 {
    let mut rng = rand::thread_rng();
    let mut r = rng.gen::<u8>();
    r = r & 0x0F;
    r
}

fn get_rand_half_octet_hex() -> String {
    let mut r = get_rand_half_octet();
    format!("{:X}", r)
}

pub fn get_rand_hex_str(bits: u8) -> String {
    let mut r = String::new();
    let half_octet = bits/4;
    for _ in 0..half_octet {
        r.push_str(&get_rand_half_octet_hex());
    }
    r
}

pub fn get_rand_dev_eui() -> String {
  get_rand_hex_str(64)
}

pub fn get_rand_app_key() -> String {
  get_rand_hex_str(128)
}

fn allow_char (c: char) -> bool {
    match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => return true,
        _ => return false,
    };
}

fn verify_hex_str(str: String, bits: u8) -> bool {
  for char in str.chars() {
    if !allow_char(char) {
      return false;
    }
  }
  let len = bits/4;
  if str.len() != len.into() {
    return false;
  }
  return true;
}

pub fn verify_app_key(str: String) -> bool {
  verify_hex_str(str, 128)
}

pub fn verify_dev_eui(str: String) -> bool {
  verify_hex_str(str, 64)
}