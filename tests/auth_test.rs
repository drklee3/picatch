use dphoto_lib::auth;
use dotenv;

#[test]
fn it_hashes_password() {
    dotenv::dotenv().ok();
    // A randomly generated password
    let password = r#"!e(>)hdJ;Z`nca9MJv)6qq?b[p;;'gb}H-Rg/c-853[BmuaSx&R\h@:[/g#DGr3#"#;
    let hash = auth::hash_password(&password).unwrap();
    println!("Hashed password: {}", &hash);
    assert!(auth::verify(&hash, &password).unwrap());
}