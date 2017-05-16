use habitat_builder_originsrv::data_store::DataStore as originsrv_datastore;
use habitat_builder_sessionsrv::data_store::DataStore as sessionsrv_datastore;
use habitat_builder_protocol as protocol;
use habitat_builder_protocol_redis as redis_protocol;
use habitat_core::package::PackageIdent;
use postgres_lib;
use protobuf;
use redis_lib;
use redis_extraction;
use regex::Regex;
use std::str::FromStr;

pub struct OriginKeyMigrator {
    origin_name: String,
    origin_keys: Vec<redis_protocol::depotsrv::OriginKeyIdent>,
    originsrv_store: originsrv_datastore,
}

impl OriginKeyMigrator {
    pub fn new(origin_name: String,
               origin_keys: Vec<redis_protocol::depotsrv::OriginKeyIdent>,
               originsrv_store: originsrv_datastore)
               -> OriginKeyMigrator {
        OriginKeyMigrator {
            origin_name: origin_name,
            origin_keys: origin_keys,
            originsrv_store: originsrv_store,
        }
    }

    pub fn migrate(&self) {
        let origin_keys = &self.origin_keys;

        let pg_origin = self.originsrv_store
            .get_origin_by_name(&self.origin_name)
            .expect("unable to get origin from postgres")
            .expect("no origin found in postgres");

        for key in origin_keys {
            if postgres_lib::get_origin_key_by_revision(self.originsrv_store.clone(),
                                                        pg_origin.get_name(),
                                                        key.get_revision())
                       .is_some() {
                return;
            }

            println!("migrating key {:?}", key);

            let mut okc = protocol::originsrv::OriginPublicKeyCreate::new();
            okc.set_name(key.get_origin().to_string());
            okc.set_revision(key.get_revision().to_string());
            okc.set_origin_id(pg_origin.get_id());
            okc.set_owner_id(pg_origin.get_owner_id());
            okc.set_body(redis_lib::get_key_body(key.get_location()).into_bytes());

            self.originsrv_store.create_origin_public_key(&okc);
        }
    }
}
