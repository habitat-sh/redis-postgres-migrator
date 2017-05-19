use habitat_builder_originsrv::data_store::DataStore as originsrv_datastore;
use habitat_builder_sessionsrv::data_store::DataStore as sessionsrv_datastore;
use postgres_lib;
use redis_lib;
use redis_extraction;
use regex::Regex;

pub struct AccountOriginMigrator {
    redis_uri: String,
    originsrv_store: originsrv_datastore,
    sessionsrv_store: sessionsrv_datastore,
}

impl AccountOriginMigrator {
    pub fn new(redis_uri: String,
               originsrv_store: originsrv_datastore,
               sessionsrv_store: sessionsrv_datastore)
               -> AccountOriginMigrator {
        AccountOriginMigrator {
            redis_uri: redis_uri,
            originsrv_store: originsrv_store,
            sessionsrv_store: sessionsrv_store,
        }
    }

    pub fn migrate(&self) {
        println!("migrating account origins...");
        let accounts = redis_extraction::extract_accounts(self.redis_uri.as_str());

        let re = Regex::new(r":(\d+)").unwrap();
        for x in accounts {
            for cap in re.captures_iter(&x) {
                let account_id = &cap[1];

                self.migrate_account(account_id.parse::<u64>().unwrap());
            }
        }
    }

    pub fn migrate_account(&self, id: u64) {
        let redis_account = redis_lib::find_account_by_id(self.redis_uri.as_str(), id.to_string());
        let account_origins = redis_lib::get_origins_by_account(self.redis_uri.as_str(),
                                                                redis_account.get_id());
        let pg_account = postgres_lib::get_account(self.sessionsrv_store.clone(),
                                                   redis_account.get_name())
                .expect("no account found for origin");

        for origin in account_origins {
            if postgres_lib::is_account_in_origin(self.sessionsrv_store.clone(),
                                                  origin.clone(),
                                                  pg_account.get_id()) {
                break;
            }

            println!("migrating redis origin {} in redis account {} to postgres account {}",
                     origin.clone(),
                     redis_account.get_name(),
                     pg_account.get_name());
            let pg_origin = self.originsrv_store
                .get_origin_by_name(origin.as_str())
                .expect("unable to get origin from postgres")
                .expect("no origin found in postgres");

            postgres_lib::create_account_origin(self.sessionsrv_store.clone(),
                                                pg_origin.get_id(),
                                                pg_origin.get_name(),
                                                pg_account.get_id(),
                                                pg_account.get_name());

        }
    }
}
