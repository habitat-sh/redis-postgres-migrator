use habitat_builder_protocol::originsrv;
use redis_postgres_migrator::migrators;
use postgres_lib;
use redis_lib;

const TEST_REDIS_ADDR: &'static str = "redis://127.0.0.1/";

#[test]
fn test_migrate() {
    let ds1 = postgres_lib::create_test_originsrv_data_store();
    let ds2 = ds1.clone();
    ds2.setup();
    let sdb1 = postgres_lib::create_test_sessionsrv_data_store();
    let sdb2 = sdb1.clone();

    let redis_session = redis_lib::create_session(String::from("token"),
                                                  64,
                                                  String::from("bobo@chef.io"),
                                                  String::from("owner name"));
    let redis_account = redis_lib::create_account(TEST_REDIS_ADDR, redis_session);
    let pg_session = postgres_lib::create_session(String::from("token"),
                                                  redis_account.get_id(),
                                                  redis_account.get_email().to_string(),
                                                  redis_account.get_name().to_string());
    let pg_account = postgres_lib::create_account(sdb1, pg_session);

    let origin1 = redis_lib::create_origin(TEST_REDIS_ADDR,
                                              "origin_name1",
                                              redis_account.get_id());
    let origin2 = redis_lib::create_origin(TEST_REDIS_ADDR,
                                              "origin_name2",
                                              redis_account.get_id());

    migrators::origin::OriginMigrator::new(TEST_REDIS_ADDR.to_string(), ds1, sdb2).migrate();

    let mut oar = originsrv::CheckOriginAccessRequest::new();
    oar.set_account_id(pg_account.get_id());
    oar.set_origin_name(origin1.get_name().to_string());
    assert!(ds2.check_account_in_origin(&oar).unwrap());

    oar.set_origin_name(origin2.get_name().to_string());
    assert!(ds2.check_account_in_origin(&oar).unwrap());
}
