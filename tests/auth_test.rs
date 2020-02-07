use dphoto_lib::auth::crypto;

// A randomly generated password
const PASSWORD: &str = r#"!e(>)hdJ;Z`nca9MJv)6qq?b[p;;'gb}H-Rg/c-853[BmuaSx&R\h@:[/g#DGr3#"#;

#[test]
fn it_hashes_and_verifies_password() {
    let hash = crypto::hash_password(&PASSWORD).unwrap();
    println!("Hashed password: {}", &hash);
    assert!(crypto::verify(&hash, &PASSWORD).unwrap());
}
