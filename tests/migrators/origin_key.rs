use redis_postgres_migrator::migrators;
use postgres_lib;
use redis_lib;
use std::path::PathBuf;

const TEST_REDIS_ADDR: &'static str = "redis://127.0.0.1:6379";

pub fn hart_file(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn test_migrate_public_key() {
    let ds = postgres_lib::create_test_originsrv_data_store();
    ds.setup();

    let origin = redis_lib::create_origin(TEST_REDIS_ADDR, "bb8", 5000);

    let pg_origin =
		postgres_lib::create_origin(ds.clone(), origin.get_name(), 5000, "account_name").unwrap();

    let revision = "20160423193732";
    let revision_2 = "20160523193732";

    let origin_key_1 = redis_lib::create_origin_key(origin.get_name(), revision, sample_public_key(), TEST_REDIS_ADDR);
    let origin_key_2 = redis_lib::create_origin_key(origin.get_name(), revision_2, sample_public_key(), TEST_REDIS_ADDR);

    let mut origin_keys = redis_lib::get_origin_keys_by_origin(origin.get_name(), TEST_REDIS_ADDR);

    // Overriding location for test purposes
    origin_keys[0].set_location("tests/fixtures/my_origin_1/my_origin_public_key.pub".to_string());
    origin_keys[1].set_location("tests/fixtures/my_origin_2/my_origin_public_key2.pub".to_string());

    assert_eq!(origin_keys.len(), 2);

    migrators::origin_key::OriginKeyMigrator::new(origin.get_name().to_string(), origin_keys, ds.clone()).migrate();

    let pg_origin_keys = postgres_lib::get_origin_keys_by_origin(ds.clone(), pg_origin.get_id());
    println!("PIKACHU!!!!");
    println!("{:?}", pg_origin_keys.get_keys());

    assert_eq!(pg_origin_keys.get_keys().len(), 2);
}

fn sample_public_key() -> String {
   "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCqGKukO1De7zhZj6+H0qtjTkVxwTCpvKe4eCZ0
FPqri0cb2JZfXJ/DgYSF6vUpwmJG8wVQZKjeGcjDOL5UlsuusFncCzWBQ7RKNUSesmQRMSGkVb1/
3j+skZ6UtW+5u09lHNsj6tQ51s1SPrCBkedbNf0Tp0GbMJDyR4e9T04ZZwIDAQAB
-----END PUBLIC KEY-----".to_string()
}
