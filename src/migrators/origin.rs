use habitat_builder_originsrv::data_store::DataStore as originsrv_datastore;
use habitat_builder_sessionsrv::data_store::DataStore as sessionsrv_datastore;
use postgres_lib;
use redis_lib;
use redis_extraction;
use regex::Regex;

pub struct OriginMigrator {
    redis_uri: String,
    originsrv_store: originsrv_datastore,
    sessionsrv_store: sessionsrv_datastore,
}

impl OriginMigrator {
    pub fn new(redis_uri: String,
               originsrv_store: originsrv_datastore,
               sessionsrv_store: sessionsrv_datastore)
               -> OriginMigrator {
        OriginMigrator {
            redis_uri: redis_uri,
            originsrv_store: originsrv_store,
            sessionsrv_store: sessionsrv_store,
        }
    }

    pub fn migrate(&self) {
        let origins = redis_extraction::extract_origins(self.redis_uri.as_str());

        let re = Regex::new(r":(\d+)").unwrap();
        for x in origins {
            for cap in re.captures_iter(&x) {
                let origin_id = &cap[1];

                self.migrate_origin(origin_id.parse::<u64>().unwrap());
            }
        }
    }

    pub fn migrate_origin(&self, id: u64) {
        let redis_origin = redis_lib::find_origin_by_id(self.redis_uri.as_str(), id);
        println!("migrating origin:{}", redis_origin.get_name());
        let redis_account = redis_lib::find_account_by_id(self.redis_uri.as_str(),
                                                          redis_origin.get_owner_id().to_string());
        let pg_account = postgres_lib::get_account(self.sessionsrv_store.clone(),
                                                   redis_account.get_name())
                .expect("no account found for origin");

        postgres_lib::create_origin(self.originsrv_store.clone(),
                                    redis_origin.get_name(),
                                    pg_account.get_id(),
                                    pg_account.get_name());
    }
}
