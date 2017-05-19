extern crate redis_postgres_migrator as migrator;
extern crate redis_lib;
extern crate postgres_lib;
extern crate redis_extraction;
extern crate regex;
use regex::Regex;

use std::env;
use migrator::migrators;

fn main() {
println!("Starting the run of the program");
    let args: Vec<_> = env::args().collect();
    let redis_address = &args[1];
    // We start with transferring accounts, which live in the builder_sessionsrv data store
    let sessionsrv_data_store = postgres_lib::create_sessionsrv_data_store();
    let originsrv_data_store = postgres_lib::create_originsrv_data_store();
    migrators::account::redis_to_postgres(redis_address, sessionsrv_data_store.clone());
    migrators::origin::OriginMigrator::new(redis_address.to_string(),
                                           originsrv_data_store.clone(),
                                           sessionsrv_data_store.clone())
            .migrate();
    migrators::account_origin::AccountOriginMigrator::new(redis_address.to_string(),
                                                  originsrv_data_store.clone(),
                                                  sessionsrv_data_store.clone())
            .migrate();
    migrators::invitation::InvitationMigrator::new(redis_address.to_string(),
                                                   originsrv_data_store.clone(),
                                                   sessionsrv_data_store.clone())
            .migrate();
    migrators::secret_key::SecretKeyMigrator::new(redis_address.to_string(),
                                                  originsrv_data_store.clone(),
                                                  sessionsrv_data_store.clone())
            .migrate();
    migrators::package::PackageMigrator::new(redis_address.to_string(),
                                             originsrv_data_store.clone())
            .migrate();
    migrate_origin_public_keys(redis_address);
}

fn migrate_origin_public_keys(redis_addr: &str) {
    let redis_origins = redis_extraction::extract_origins(redis_addr);

    let originsrv_data_store = postgres_lib::create_originsrv_data_store();

    let re = Regex::new(r":(\d+)").unwrap();
    for x in redis_origins {
        for cap in re.captures_iter(&x) {
            let origin_id = &cap[1];
            let redis_origin = redis_lib::find_origin_by_id(redis_addr,
                                                            origin_id.parse::<u64>().unwrap());
            let origin_keys = redis_lib::get_origin_keys_by_origin(redis_origin.get_name(),
                                                                   redis_addr);

            migrators::origin_key::OriginKeyMigrator::new(redis_origin.get_name().to_string(),
                                                          origin_keys,
                                                          originsrv_data_store.clone())
                    .migrate();
        }
    }
}
