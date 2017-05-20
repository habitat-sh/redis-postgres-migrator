use habitat_builder_originsrv::data_store::DataStore as originsrv_datastore;
use habitat_builder_sessionsrv::data_store::DataStore as sessionsrv_datastore;
use postgres_lib;
use redis_lib;
use redis_extraction;
use regex::Regex;

pub struct OriginMemberMigrator {
    redis_uri: String,
    originsrv_store: originsrv_datastore,
    sessionsrv_store: sessionsrv_datastore,
}

impl OriginMemberMigrator {
    pub fn new(redis_uri: String,
               originsrv_store: originsrv_datastore,
               sessionsrv_store: sessionsrv_datastore)
               -> OriginMemberMigrator {
        OriginMemberMigrator {
            redis_uri: redis_uri,
            originsrv_store: originsrv_store,
            sessionsrv_store: sessionsrv_store,
        }
    }

    pub fn migrate(&self) {
        println!("migrating origins accounts...");
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
        let origin_members = redis_lib::get_origins_members(self.redis_uri.as_str(),
                                                            redis_origin.get_id());
        let pg_origin = postgres_lib::get_origin_by_name(self.originsrv_store.clone(),
                                                         redis_origin.get_name())
                .expect("no origin found found for origin name");

        for member in origin_members {
            if postgres_lib::is_member_in_origin(self.originsrv_store.clone(),
                                                 member.clone(),
                                                 pg_origin.get_id()) {
                continue;
            }

            println!("migrating member {} to origin {}",
                     member.clone(),
                     redis_origin.get_name());

            let pg_account = postgres_lib::get_account(self.sessionsrv_store.clone(),
                                                       member.as_str())
                    .expect("no account found for account name");

            self.originsrv_store
                .clone()
                .create_origin_member(pg_origin.get_id(),
                                      pg_origin.get_name().to_string(),
                                      pg_account.get_id(),
                                      pg_account.get_name().to_string());

        }
    }
}
