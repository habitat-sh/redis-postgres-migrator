use redis_postgres_migrator::migrators;
use habitat_builder_protocol as protocol;
use postgres_lib;
use redis_lib;
use std::path::PathBuf;

const TEST_REDIS_ADDR: &'static str = "redis://127.0.0.1:6379";

#[test]
fn test_migrate_secret_key() {
    let ds = postgres_lib::create_test_originsrv_data_store();
    ds.setup();
    let sds = postgres_lib::create_test_sessionsrv_data_store();
    sds.setup();

    let redis_session1 = redis_lib::create_session(String::from("token1"),
                                                   5000,
                                                   String::from("bobo@chef.io"),
                                                   String::from("account1"));
    let redis_account1 = redis_lib::create_account(TEST_REDIS_ADDR, redis_session1.clone());
    let origin1 = redis_lib::create_origin(TEST_REDIS_ADDR, "bb8", redis_account1.get_id());
    let pg_session1 = postgres_lib::create_session(redis_session1.get_token().to_string(),
                                                   5000,
                                                   redis_session1.get_email().to_string(),
                                                   redis_session1.get_name().to_string());
    let pg_account1 = postgres_lib::create_account(sds.clone(), pg_session1);
    let pg_origin1 = postgres_lib::create_origin(ds.clone(),
                                                 origin1.get_name(),
                                                 pg_account1.get_id(),
                                                 pg_account1.get_name())
            .unwrap();

    let redis_session2 = redis_lib::create_session(String::from("token2"),
                                                   5001,
                                                   String::from("bobo@chef.io"),
                                                   String::from("account2"));
    let redis_account2 = redis_lib::create_account(TEST_REDIS_ADDR, redis_session2.clone());
    let pg_session2 = postgres_lib::create_session(redis_session2.get_token().to_string(),
                                                   5001,
                                                   redis_session2.get_email().to_string(),
                                                   redis_session2.get_name().to_string());
    let pg_account2 = postgres_lib::create_account(sds.clone(), pg_session2);
    let origin2 = redis_lib::create_origin(TEST_REDIS_ADDR, "bb9", redis_account2.get_id());
    let pg_origin2 = postgres_lib::create_origin(ds.clone(),
                                                 origin2.get_name(),
                                                 pg_account2.get_id(),
                                                 pg_account2.get_name())
            .unwrap();

    let redis_session3 = redis_lib::create_session(String::from("token3"),
                                                   5002,
                                                   String::from("bobo@chef.io"),
                                                   String::from("account3"));
    let redis_account3 = redis_lib::create_account(TEST_REDIS_ADDR, redis_session3.clone());
    let pg_session3 = postgres_lib::create_session(redis_session3.get_token().to_string(),
                                                   5002,
                                                   redis_session3.get_email().to_string(),
                                                   redis_session3.get_name().to_string());
    let pg_account3 = postgres_lib::create_account(sds.clone(), pg_session3);
    let origin3 = redis_lib::create_origin(TEST_REDIS_ADDR, "bb7", redis_account3.get_id());
    let pg_origin3 = postgres_lib::create_origin(ds.clone(),
                                                 origin3.get_name(),
                                                 pg_account3.get_id(),
                                                 pg_account3.get_name())
            .unwrap();

    let secret_key_1 = redis_lib::create_secret_key(origin1.get_id(),
                                                    origin1.get_name(),
                                                    "revision1",
                                                    "body1",
                                                    redis_account1.get_id(),
                                                    TEST_REDIS_ADDR);
    let secret_key_2 = redis_lib::create_secret_key(origin2.get_id(),
                                                    origin2.get_name(),
                                                    "revision2",
                                                    "body2",
                                                    redis_account2.get_id(),
                                                    TEST_REDIS_ADDR);
    let secret_key_3 = redis_lib::create_secret_key(origin3.get_id(),
                                                    origin3.get_name(),
                                                    "revision3",
                                                    "body3",
                                                    redis_account3.get_id(),
                                                    TEST_REDIS_ADDR);

    //manually insert a key to simulate dupes
    let mut osk = protocol::originsrv::OriginSecretKeyCreate::new();
    osk.set_body(secret_key_3.get_body().to_vec());
    osk.set_name(secret_key_3.get_name().to_string());
    osk.set_origin_id(pg_origin3.get_id());
    osk.set_owner_id(pg_account3.get_id());
    osk.set_revision(secret_key_3.get_revision().to_string());
    ds.create_origin_secret_key(&osk);

    migrators::secret_key::SecretKeyMigrator::new(TEST_REDIS_ADDR.to_string(),
                                                  ds.clone(),
                                                  sds.clone())
            .migrate();

    let pg_secret_key1 = postgres_lib::get_secret_key_by_origin(ds.clone(), pg_origin1.get_name())
        .expect("did not get first key");
    assert_eq!(pg_origin1.get_id(), pg_secret_key1.get_origin_id());
    assert_eq!(secret_key_1.get_name(), pg_secret_key1.get_name());
    assert_eq!(secret_key_1.get_revision(), pg_secret_key1.get_revision());
    assert_eq!(secret_key_1.get_body(), pg_secret_key1.get_body());
    assert_eq!(pg_account1.get_id(), pg_secret_key1.get_owner_id());

    let pg_secret_key2 = postgres_lib::get_secret_key_by_origin(ds.clone(), pg_origin2.get_name())
        .expect("did not get second key");
    assert_eq!(pg_origin2.get_id(), pg_secret_key2.get_origin_id());
    assert_eq!(secret_key_2.get_name(), pg_secret_key2.get_name());
    assert_eq!(secret_key_2.get_revision(), pg_secret_key2.get_revision());
    assert_eq!(secret_key_2.get_body(), pg_secret_key2.get_body());
    assert_eq!(pg_account2.get_id(), pg_secret_key2.get_owner_id());

    let pg_secret_key3 = postgres_lib::get_secret_key_by_origin(ds.clone(), pg_origin3.get_name())
        .expect("did not get third key");
    assert_eq!(pg_origin3.get_id(), pg_secret_key3.get_origin_id());
    assert_eq!(secret_key_3.get_name(), pg_secret_key3.get_name());
    assert_eq!(secret_key_3.get_revision(), pg_secret_key3.get_revision());
    assert_eq!(secret_key_3.get_body(), pg_secret_key3.get_body());
    assert_eq!(pg_account3.get_id(), pg_secret_key3.get_owner_id());
}
