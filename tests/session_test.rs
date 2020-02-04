use chrono::Utc;
use dphoto_lib::model::store::Session;

#[test]
fn it_generates_random_session_id() {
    let now = Utc::now().naive_utc();
    let user_agent = Some("Test user agent".to_owned());

    let ses1 = Session::new(1, None, user_agent.clone(), now.clone());
    let ses2 = Session::new(2, None, user_agent.clone(), now.clone());
    assert!(ses1.id != ses2.id);
    println!("{}\n{}", ses1.id, ses2.id);
}
