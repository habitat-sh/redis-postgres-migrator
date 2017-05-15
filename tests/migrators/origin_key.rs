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
fn test_migrate_origin_key() {
    let ds = postgres_lib::create_test_originsrv_data_store();
    ds.setup();

    let origin = redis_lib::create_origin(TEST_REDIS_ADDR, "bb8", 5000);

    let pg_origin =
		postgres_lib::create_origin(ds.clone(), origin.get_name(), 5000, "account_name").unwrap();

    let revision = "20160423193732";
    let origin_key_1 = redis_lib::create_origin_key(origin.get_name(), revision, sample_public_key(), TEST_REDIS_ADDR);

    let origin_keys = redis_lib::list_origin_keys(origin.get_name(), TEST_REDIS_ADDR);
}

fn sample_public_key() -> String {
   "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCqGKukO1De7zhZj6+H0qtjTkVxwTCpvKe4eCZ0
FPqri0cb2JZfXJ/DgYSF6vUpwmJG8wVQZKjeGcjDOL5UlsuusFncCzWBQ7RKNUSesmQRMSGkVb1/
3j+skZ6UtW+5u09lHNsj6tQ51s1SPrCBkedbNf0Tp0GbMJDyR4e9T04ZZwIDAQAB
-----END PUBLIC KEY-----".to_string()
}
