use habitat_builder_originsrv::data_store::DataStore as originsrv_datastore;
use habitat_builder_sessionsrv::data_store::DataStore as sessionsrv_datastore;
use habitat_builder_protocol as protocol;
use habitat_builder_protocol_redis;
use habitat_core::package::PackageIdent;
use postgres_lib;
use protobuf;
use redis_lib;
use redis_extraction;
use regex::Regex;
use std::str::FromStr;

pub struct SecretKeyMigrator {
    redis_uri: String,
    originsrv_store: originsrv_datastore,
    sessionsrv_store: sessionsrv_datastore,
}

impl SecretKeyMigrator {
    pub fn new(redis_uri: String,
               originsrv_store: originsrv_datastore,
               sessionsrv_store: sessionsrv_datastore)
               -> SecretKeyMigrator {
        SecretKeyMigrator {
            redis_uri: redis_uri,
            originsrv_store: originsrv_store,
            sessionsrv_store: sessionsrv_store,
        }
    }

    pub fn migrate(&self) {
        let keys = redis_extraction::extract_secret_keys(self.redis_uri.as_str());

        let re = Regex::new(r":(\d+)").unwrap();
        for x in keys {
            for cap in re.captures_iter(&x) {
                let key_id = &cap[1];

                self.migrate_secret_key(key_id.parse::<u64>().unwrap());
            }
        }
    }

    fn migrate_secret_key(&self, id: u64) {
        let key = redis_lib::get_secret_key_by_id(self.redis_uri.as_str(), id);
        let redis_origin = redis_lib::find_origin_by_id(self.redis_uri.as_str(),
                                                        key.get_origin_id());

        if postgres_lib::get_secret_key_by_origin(self.originsrv_store.clone(),
                                                  redis_origin.get_name())
                   .is_some() {
            return;
        }

        let pg_origin = self.originsrv_store
            .get_origin_by_name(redis_origin.get_name())
            .expect("unable to get origin from postgres")
            .expect("no origin found in postgres");
        let redis_account = redis_lib::find_account_by_id(self.redis_uri.as_str(),
                                                          redis_origin.get_owner_id().to_string());
        let account = postgres_lib::get_account(self.sessionsrv_store.clone(),
                                                redis_account.get_name())
                .expect("no account found for origin");
        println!("migrating secret key {} for origin:{}",
                 redis_origin.get_name(),
                 key.get_name());
        let mut osk = protocol::originsrv::OriginSecretKeyCreate::new();
        osk.set_body(key.get_body().to_vec());
        osk.set_name(key.get_name().to_string());
        osk.set_origin_id(pg_origin.get_id());
        osk.set_owner_id(account.get_id());
        osk.set_revision(key.get_revision().to_string());
        self.originsrv_store
            .create_origin_secret_key(&osk)
            .expect("error adding key");
    }
}
