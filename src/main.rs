extern crate redis_postgres_migrator as migrator;
extern crate postgres_lib;

use std::env;
use migrator::migrators;

fn main() {
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
    migrators::invitation::InvitationMigrator::new(redis_address.to_string(),
                                           originsrv_data_store.clone(),
                                           sessionsrv_data_store.clone())
            .migrate();
    migrators::package::PackageMigrator::new(redis_address.to_string(),
                                             originsrv_data_store.clone())
            .migrate();
}
