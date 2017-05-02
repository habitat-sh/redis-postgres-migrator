//use hab_sessionsrv as hab_sessionsrv;

pub fn create_session(token: String, extern_id: u64, email: String, name: String) -> protocol::sessionsrv::SessionCreate {
    let mut sc = protocol::sessionsrv::SessionCreate::new();
    sc.set_token(token);
    sc.set_extern_id(extern_id);
    sc.set_email(email);
    sc.set_name(String::from(name));
    sc.set_provider(protocol::sessionsrv::OAuthProvider::GitHub);
    sc
}

pub fn create_account() {
    let config = hab_sessionsrv::config::Config::default;
    let ds = hab_sessionsrv::data_store::DataStore::new(config);
}
