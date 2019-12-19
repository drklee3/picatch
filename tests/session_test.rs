use dphoto_lib::model::store::Session;
use chrono::Utc;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn it_generates_random_session_id() {
    let ip = IpAddr::from(Ipv4Addr::new(192, 168, 1, 1));
    let now = Utc::now().naive_utc();
    let user_agent = "Test user agent".to_owned();

    let ses1 = Session::new(1, ip, user_agent.clone(), now.clone());
    let ses2 = Session::new(2, ip, user_agent.clone(), now.clone());
    assert!(ses1.id != ses2.id);
    println!("{}\n{}", ses1.id, ses2.id);
}