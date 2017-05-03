extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
extern crate habitat_builder_db as hab_db;

pub fn create_session(token: String, extern_id: u64, email: String, name: String) -> protocol::sessionsrv::SessionCreate {
    let mut sc = protocol::sessionsrv::SessionCreate::new();
    sc.set_token(token);
    sc.set_extern_id(extern_id);
    sc.set_email(email);
    sc.set_name(String::from(name));
    sc.set_provider(protocol::sessionsrv::OAuthProvider::GitHub);
    sc
}

pub fn create_account(session: protocol::sessionsrv::SessionCreate) -> protocol::sessionsrv::Session {
    let config = hab_sessionsrv::config::Config::default();
    let data_store = hab_sessionsrv::data_store::DataStore::new(&config).unwrap();
    let account_creation = data_store.find_or_create_account_via_session(&session, false, false, false);
    let account = account_creation.unwrap();
    account
}
