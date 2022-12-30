

pub fn get_md5(raw: &String) -> String {
    let hashable = raw.as_bytes();

    format!("{:x}", md5::compute(hashable))
}

pub fn hash(pw_raw: &String) -> String {
    let pw_md5 = get_md5(pw_raw);

    bcrypt::hash(pw_md5, bcrypt::DEFAULT_COST).unwrap()
}

pub fn verify(pw_md5: &String, pw_bcrypt: &String) -> bool {
    bcrypt::verify(pw_md5, pw_bcrypt).unwrap()
}