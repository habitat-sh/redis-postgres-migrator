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

pub struct InvitationMigrator {
    redis_uri: String,
    originsrv_store: originsrv_datastore,
    sessionsrv_store: sessionsrv_datastore,
}

impl InvitationMigrator {
    pub fn new(redis_uri: String,
               originsrv_store: originsrv_datastore,
               sessionsrv_store: sessionsrv_datastore)
               -> InvitationMigrator {
        InvitationMigrator {
            redis_uri: redis_uri,
            originsrv_store: originsrv_store,
            sessionsrv_store: sessionsrv_store,
        }
    }

    pub fn migrate(&self) {
        println!("migrating invitations...");
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
        let origin = self.originsrv_store
            .get_origin_by_name(redis_origin.get_name())
            .expect("unable to get origin from postgres")
            .expect("no origin found in postgres");
        for invite in redis_lib::get_invitations_by_origin(self.redis_uri.as_str(),
                                                           redis_origin.get_id()) {
            let invitee = postgres_lib::get_account(self.sessionsrv_store.clone(),
                                                    invite.get_account_name())
                    .expect("unable to get invitee account");
            let redis_owner = redis_lib::find_account_by_id(self.redis_uri.as_str(),
                                                            invite.get_owner_id().to_string());
            let owner = postgres_lib::get_account(self.sessionsrv_store.clone(),
                                                  redis_owner.get_name())
                    .expect("unable to get owner account");

            let mut oic = protocol::originsrv::OriginInvitationCreate::new();
            oic.set_account_id(invitee.get_id());
            oic.set_account_name(invitee.get_name().to_string());
            oic.set_origin_id(origin.get_id());
            oic.set_origin_name(origin.get_name().to_string());
            oic.set_owner_id(owner.get_id());
            if self.originsrv_store.create_origin_invitation(&oic).expect("failed to add invitation").is_some() {
                println!("migrated invitation for {} to join {}",
                        invite.get_account_name(),
                        invite.get_origin_name());
            }
        }
    }
}
