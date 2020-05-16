use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref PHOTOS_DIR: String =
        env::var("PICATCH_PHOTOS_DIR").unwrap_or("./photos".to_string());
    pub static ref PUBLIC_DIR: String =
        env::var("PICATCH_PUBLIC_DIR").unwrap_or("./web/build".to_string());
}
