extern crate redis_postgres_migrator;
extern crate iron;
extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as hab_protocol;
extern crate habitat_builder_dbcache as hab_dbcache;
extern crate habitat_net as hab_net;
extern crate protobuf;

use std::any::TypeId;
use std::collections::HashMap;
use hab_protocol::net::{NetOk, ErrCode, NetError};
use protobuf::*;
use hab_protocol::Routable;
use hab_net::routing::{Broker, RouteResult};
use iron::typemap::Key;

fn main() {
    println!("bite me");
}

pub fn redis_to_postgres(thing: String) {
// redis_postgres_migrator::builder-sessionsrv::data_store::
    println!("{}", thing);
}

#[derive(Default)]
pub struct TestableBroker {
    message_map: HashMap<TypeId, Vec<u8>>,
    error_map: HashMap<TypeId, NetError>,
    cached_messages: HashMap<TypeId, Vec<u8>>,
}

impl TestableBroker {
    pub fn setup<M: Routable, R: protobuf::MessageStatic>(&mut self, response: &R) {
        let bytes = response.write_to_bytes().unwrap();
        self.message_map.insert(TypeId::of::<M>(), bytes);
    }

    pub fn setup_error<M: Routable>(&mut self, error: NetError) {
        self.error_map.insert(TypeId::of::<M>(), error);
    }

    pub fn routed_messages(&self) -> RoutedMessages {
        RoutedMessages(self.cached_messages.clone())
    }

    pub fn route<M: Routable, R: protobuf::MessageStatic>(&mut self, msg: &M) -> RouteResult<R> {
        let bytes = msg.write_to_bytes().unwrap();
        self.cached_messages.insert(TypeId::of::<M>(), bytes);
        let msg_type = &TypeId::of::<M>();
        match self.message_map.get(msg_type) {
            Some(message) => Ok(parse_from_bytes::<R>(message).unwrap()),
            None => {
                match self.error_map.get(msg_type) {
                    Some(error) => Err(error.clone()),
                    None => panic!("Unable to find message of given type"),
                }
            }
        }
    }
}

pub struct RoutedMessages(HashMap<TypeId, Vec<u8>>);


impl Key for TestableBroker {
    type Value = Self;
}

impl RoutedMessages {
    pub fn get<M: Routable>(&self) -> Result<M> {
        let msg_type = &TypeId::of::<M>();
        match self.0.get(msg_type) {
            Some(msg) => {
                Ok(parse_from_bytes::<M>(msg).expect(&format!("Unable to parse {:?} message",
                                                              msg_type)))
            }
            None => Err(Error::MessageTypeNotFound),
        }
    }
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
