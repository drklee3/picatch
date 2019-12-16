use dphoto_lib::auth;
use std::env;

// This is an example secret key for testing, it's not used anywhere and don't use it yourself obviously
const SECRET_KEY: &str = r#"R77hK1wKrNt4FZtcXGnhstDs43A7dMyC"#;
// A randomly generated password
const PASSWORD: &str = r#"!e(>)hdJ;Z`nca9MJv)6qq?b[p;;'gb}H-Rg/c-853[BmuaSx&R\h@:[/g#DGr3#"#;

#[test]
fn it_hashes_and_verifies_password() {
    env::set_var("SECRET_KEY", &SECRET_KEY);
    let hash = auth::hash_password(&PASSWORD).unwrap();
    println!("Hashed password: {}", &hash);
    assert!(auth::verify(&hash, &PASSWORD).unwrap());
}
