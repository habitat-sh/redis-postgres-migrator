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

pub struct PackageMigrator {
    redis_uri: String,
    originsrv_store: originsrv_datastore,
}

impl PackageMigrator {
    pub fn new(redis_uri: String, originsrv_store: originsrv_datastore) -> PackageMigrator {
        PackageMigrator {
            redis_uri: redis_uri,
            originsrv_store: originsrv_store,
        }
    }

    pub fn migrate(&self) {
        println!("migrating packages...");
        let origins = redis_extraction::extract_origins(self.redis_uri.as_str());

        let re = Regex::new(r":(\d+)").unwrap();
        for x in origins {
            for cap in re.captures_iter(&x) {
                let origin_id = &cap[1];

                self.migrate_origin(origin_id.parse::<u64>().unwrap());
            }
        }
    }

    fn migrate_origin(&self, id: u64) {
        let redis_origin = redis_lib::find_origin_by_id(self.redis_uri.as_str(), id);
        let pg_origin = self.originsrv_store
            .get_origin_by_name(redis_origin.get_name())
            .expect("unable to get origin from postgres")
            .expect("no origin found in postgres");
        for ident in redis_lib::get_package_idents_by_origin(self.redis_uri.as_str(),
                                                             redis_origin.get_name()) {
            if postgres_lib::get_package_by_ident(self.originsrv_store.clone(), ident.to_string().as_str())
                   .is_some() {
                return;
            }

            println!("migrating package :{}", ident.to_string());
            let package = redis_lib::get_package_by_ident(self.redis_uri.as_str(), ident);
            let mut opc = protocol::originsrv::OriginPackageCreate::new();
            opc.set_checksum(package.get_checksum().to_string());
            opc.set_config(package.get_config().to_string());
            opc.set_deps(self.repeated_deps(package.get_deps()));
            opc.set_exposes(package.get_exposes().to_vec());
            opc.set_ident(PackageIdent::from_str(package.get_ident().to_string().as_str())
                              .unwrap()
                              .into());
            opc.set_manifest(package.get_manifest().to_string());
            opc.set_tdeps(self.repeated_deps(package.get_tdeps()));
            opc.set_target("x86_64-linux".to_string());
            opc.set_origin_id(pg_origin.get_id());
            opc.set_owner_id(pg_origin.get_owner_id());
            self.originsrv_store
                .create_origin_package(&opc)
                .expect("failed to save package");
        }
    }

    fn repeated_deps
        (&self,
         idents: &[habitat_builder_protocol_redis::depotsrv::PackageIdent])
         -> protobuf::repeated::RepeatedField<protocol::originsrv::OriginPackageIdent> {
        let mut deps = protobuf::RepeatedField::new();
        for ident in idents {
            deps.push(PackageIdent::from_str(ident.to_string().as_str())
                          .unwrap()
                          .into());
        }
        deps
    }
}
