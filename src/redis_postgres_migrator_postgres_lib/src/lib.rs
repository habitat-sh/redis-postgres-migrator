extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
#[macro_use]
extern crate habitat_builder_db as hab_db;
use hab_sessionsrv::data_store::DataStore as sessionsrv_data_store;
use protocol::sessionsrv::Session;
use hab_db::pool::Pool;
use std::ops::Deref;


pub fn create_session(token: String, extern_id: u64, email: String, name: String) -> protocol::sessionsrv::SessionCreate {
    let mut sc = protocol::sessionsrv::SessionCreate::new();
    sc.set_token(token);
    sc.set_extern_id(extern_id);
    sc.set_email(email);
    sc.set_name(String::from(name));
    sc.set_provider(protocol::sessionsrv::OAuthProvider::GitHub);
    sc
}


pub fn create_account(data_store: sessionsrv_data_store, session: protocol::sessionsrv::SessionCreate) -> protocol::sessionsrv::Session {
    let config = hab_sessionsrv::config::Config::default();
//    let conn = data_store.pool.get(&session)?;

//    let rows = conn.query("SELECT * FROM select_or_insert_account_v1($1, $2)",
 //                         &[&session_create.get_name(), &session_create.get_email()])
//        .map_err(Error::AccountCreate)?;

//println!("===========");
//println!("===========");
//println!("{:?}", &session);
    let account_creation = data_store.find_or_create_account_via_session(&session, false, false, false);
    let account = account_creation.unwrap();
    account
}

pub fn get_account(data_store: sessionsrv_data_store, account_name: &str) -> std::option::Option<protocol::sessionsrv::Account> {
//println!("in the get account function in postgres_lib");
//println!("==================");
//println!("{:?}", account_name);
    let mut ag = protocol::sessionsrv::AccountGet::new();
    ag.set_name(account_name.to_string());
    let account_get = data_store.get_account(&ag);
    let account = account_get.unwrap();
//println!("here is the account get");
//println!("{:?}", account);
    account
}

pub fn create_test_data_store() -> sessionsrv_data_store {
    let ds = datastore_test!(sessionsrv_data_store);
    ds
}
