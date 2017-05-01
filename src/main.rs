extern crate redis_postgres_migrator;
extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as hab_protocol;
extern crate habitat_builder_dbcache as hab_dbcache;
extern crate habitat_net as hab_net;

fn main() {
    println!("bite me");
}

pub fn redis_to_postgres(thing: String) {
// redis_postgres_migrator::builder-sessionsrv::data_store::
    println!("{}", thing);
}

#[cfg(test)]
mod tests {
//    use super::*;
		use hab_protocol::sessionsrv as proto_session;
    use hab_sessionsrv::data_store as sessionsrv_data_store;
    use hab_dbcache::data_store as dbcache_data_store;
    use hab_sessionsrv::config as session_srv_config;
    use hab_net::routing::{Broker, BrokerConn};


//    fn test_creating_data() {

//    }

#[test]
fn create_account() {
    let config = session_srv_config::Config::default();

    let mut sc = proto_session::SessionCreate::new();
    sc.set_token(String::from("hail2theking"));
    sc.set_extern_id(64);
    sc.set_email(String::from("bobo@chef.io"));
    sc.set_name(String::from("Bobo T. Clown"));
    sc.set_provider(proto_session::OAuthProvider::GitHub);


    let mut conn = Broker::connect().unwrap();
    let session = conn.route::<proto_session::SessionCreate, proto_session::Session>(&sc);

println!("==================");
println!("{:?}", session)

//    let sessionsrv_datastore = sessionsrv_data_store::DataStore

//    let ds = hab_sessionsrv::data_store::new();
//    let pool :ConnectionPool = Pool::start(&config);
//let pool :Arc<ConnectionPool> = Pool::start(&config);
//    let account = sessionsrv_data_store::AccountTable::new(pool);


//    let mut account = proto_session::Account::new();
//    account.set_email(sc.get_email().to_string());
//    account.set_name(sc.get_name().to_string());
//    sessionsrv_data_store.write(&mut account);
//    hab_sessionsrv::data_store::DataStore.write(&mut account);

//    DataStore.session_create(&sc);
//let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");

 //   let session = ds.find_or_create_account_via_session(&sc, true, false, false)
 //       .expect("Should create account");
 //   assert!(session.get_id() != 0, "Created account has an ID");
 //   assert_eq!(session.get_email(), "bobo@chef.io");
 //   assert_eq!(session.get_name(), "Bobo T. Clown");

  //  let session2 = ds.find_or_create_account_via_session(&sc, true, false, false)
  //      .expect("Should return account");
 //   assert_eq!(session.get_id(), session2.get_id());
  //  assert_eq!(session.get_email(), session2.get_email());
   // assert_eq!(session.get_name(), session2.get_name());
}


//    fn test_data_transfer() {
//        redis_to_postgres("Bite me!".to_string())
//   }
}
