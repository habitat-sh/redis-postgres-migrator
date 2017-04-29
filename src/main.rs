extern crate redis_postgres_migrator;
extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as hab_protocol;

fn main() {
    println!("bite me");
}

pub fn redis_to_postgres(thing: String) {
// redis_postgres_migrator::builder-sessionsrv::data_store::
    println!("{}", thing);
}

#[cfg(test)]
mod tests {
    use super::*;
		use hab_protocol::sessionsrv as proto_session;

//    fn test_creating_data() {

//    }

#[test]
fn create_account() {
//    let ds = datastore_test!(DataStore);
    let mut sc = proto_session::SessionCreate::new();
//    sc.set_token(String::from("hail2theking"));
 //   sc.set_extern_id(64);
//    sc.set_email(String::from("bobo@chef.io"));
//    sc.set_name(String::from("Bobo T. Clown"));
//    sc.set_provider(sessionsrv::OAuthProvider::GitHub);

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
