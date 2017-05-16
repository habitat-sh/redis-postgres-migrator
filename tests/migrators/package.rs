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
fn test_migrate_package() {
    let ds = postgres_lib::create_test_originsrv_data_store();
    ds.setup();

    let origin = redis_lib::create_origin(TEST_REDIS_ADDR, "core", 5000);

    let pg_origin =
        postgres_lib::create_origin(ds.clone(), origin.get_name(), 5000, "account_name").unwrap();

    let package1 = redis_lib::create_package(TEST_REDIS_ADDR,
                                             hart_file("core-libossp-uuid-1.6.2-20161214075157-x86_64-linux.hart"));
    migrators::package::PackageMigrator::new(TEST_REDIS_ADDR.to_string(), ds.clone()).migrate();

    let package2 = redis_lib::create_package(TEST_REDIS_ADDR,
                                             hart_file("core-libcap-2.24-20161208223353-x86_64-linux.hart"));
    let package3 = redis_lib::create_package(TEST_REDIS_ADDR,
                                             hart_file("core-busybox-static-1.24.2-20161214032531-x86_64-linux.hart"));

    migrators::package::PackageMigrator::new(TEST_REDIS_ADDR.to_string(), ds.clone()).migrate();

    let pg_package1 = postgres_lib::get_package_by_ident(ds.clone(),
                                                         "core/libossp-uuid/1.6.2/20161214075157")
            .expect("could not get libossp");
    let pg_package2 = postgres_lib::get_package_by_ident(ds.clone(),
                                                         "core/libcap/2.24/20161208223353")
            .expect("could not get libcap");
    let pg_package3 =
        postgres_lib::get_package_by_ident(ds.clone(), "core/busybox-static/1.24.2/20161214032531")
            .expect("could not get busybox");

    assert_eq!(package1.get_ident().to_string(),
               pg_package1.get_ident().to_string());
    assert_eq!(package1.get_checksum(), pg_package1.get_checksum());
    assert_eq!(package1.get_manifest(), pg_package1.get_manifest());
    assert_eq!(package1.get_exposes(), pg_package1.get_exposes());
    assert_eq!(package1.get_config(), pg_package1.get_config());
    assert_eq!("x86_64-linux", pg_package1.get_target());
    assert_eq!(pg_origin.get_id(), pg_package1.get_origin_id());
    assert_eq!(5000, pg_package1.get_owner_id());

    assert_eq!(package2.get_ident().to_string(),
               pg_package2.get_ident().to_string());
    assert_eq!(package2.get_checksum(), pg_package2.get_checksum());
    assert_eq!(package2.get_manifest(), pg_package2.get_manifest());
    assert_eq!(package2.get_exposes(), pg_package2.get_exposes());
    assert_eq!(package2.get_config(), pg_package2.get_config());
    assert_eq!("x86_64-linux", pg_package2.get_target());
    assert_eq!(pg_origin.get_id(), pg_package2.get_origin_id());
    assert_eq!(5000, pg_package2.get_owner_id());

    assert_eq!(package3.get_ident().to_string(),
               pg_package3.get_ident().to_string());
    assert_eq!(package3.get_checksum(), pg_package3.get_checksum());
    assert_eq!(package3.get_manifest(), pg_package3.get_manifest());
    assert_eq!(package3.get_exposes(), pg_package3.get_exposes());
    assert_eq!(package3.get_config(), pg_package3.get_config());
    assert_eq!("x86_64-linux", pg_package3.get_target());
    assert_eq!(pg_origin.get_id(), pg_package3.get_origin_id());
    assert_eq!(5000, pg_package3.get_owner_id());
}
