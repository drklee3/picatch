pub mod album;
pub mod image;
pub mod index;
pub mod login;
pub mod logout;
pub mod register;
pub mod user;

pub use self::album::get_album;
pub use self::image::get_image;
pub use self::index::get_index;
pub use self::login::post_login;
pub use self::logout::post_logout;
pub use self::register::post_register;
pub use self::user::get_username_exists;
pub use self::user::get_current_user;
