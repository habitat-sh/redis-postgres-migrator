extern crate habitat_builder_sessionsrv as hab_sessionsrv;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
extern crate habitat_builder_originsrv as hab_originsrv;

#[macro_use]
extern crate habitat_builder_db as hab_db;

use hab_sessionsrv::data_store::DataStore as sessionsrv_data_store;
use hab_originsrv::data_store::DataStore as originsrv_data_store;

use hab_core::package::PackageIdent;
use protocol::sessionsrv::Session;
use hab_db::pool::Pool;
use std::env;
use std::ops::Deref;
use std::str::FromStr;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate num_cpus;
use std::net::{Ipv4Addr, IpAddr};
use std::time::Duration;
use std::error;
use std::thread;

use postgres::Connection;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use postgres::params::IntoConnectParams;
use r2d2::ManageConnection;

pub fn create_session(token: String,
                      extern_id: u64,
                      email: String,
                      name: String)
                      -> protocol::sessionsrv::SessionCreate {
    let mut sc = protocol::sessionsrv::SessionCreate::new();
    sc.set_token(token);
    sc.set_extern_id(extern_id);
    sc.set_email(email);
    sc.set_name(String::from(name));
    sc.set_provider(protocol::sessionsrv::OAuthProvider::GitHub);
    sc
}


pub fn create_account(data_store: sessionsrv_data_store,
                      session: protocol::sessionsrv::SessionCreate)
                      -> protocol::sessionsrv::Session {
    let config = hab_sessionsrv::config::Config::default();
    let account_creation =
        data_store.find_or_create_account_via_session(&session, false, false, false);
    let account = account_creation.unwrap();
    account
}

pub fn create_origin(data_store: originsrv_data_store,
                     name: &str,
                     owner_id: u64,
                     owner_name: &str)
                     -> Option<protocol::originsrv::Origin> {
    let mut oc = protocol::originsrv::OriginCreate::new();
    oc.set_name(name.to_string());
    oc.set_owner_id(owner_id);
    oc.set_owner_name(owner_name.to_string());
    data_store
        .create_origin(&oc)
        .expect("error saving origin")
}

pub fn get_package_by_ident(data_store: originsrv_data_store,
                            ident: &str)
                            -> Option<protocol::originsrv::OriginPackage> {
    let mut opg = protocol::originsrv::OriginPackageGet::new();
    opg.set_ident(PackageIdent::from_str(ident).unwrap().into());
    data_store
        .get_origin_package(&opg)
        .expect("unable to get package from postgres")
}

pub fn get_account(data_store: sessionsrv_data_store,
                   account_name: &str)
                   -> std::option::Option<protocol::sessionsrv::Account> {
    let mut ag = protocol::sessionsrv::AccountGet::new();
    ag.set_name(account_name.to_string());
    let account_get = data_store.get_account(&ag);
    let account = account_get.unwrap();
    account
}

pub fn get_origin_by_name(data_store: originsrv_data_store,
                          origin_name: &str)
                          -> std::option::Option<protocol::originsrv::Origin> {
    let mut og = protocol::originsrv::OriginGet::new();
    og.set_name(origin_name.to_string());
    data_store.get_origin(&og).expect("cant get origin yo")
}

pub fn get_invitations_by_origin(data_store: originsrv_data_store,
                                 origin_id: u64)
                                 -> protocol::originsrv::OriginInvitationListResponse {
    let mut oilr = protocol::originsrv::OriginInvitationListRequest::new();
    oilr.set_origin_id(origin_id);
    data_store
        .list_origin_invitations_for_origin(&oilr)
        .expect("unable to get invitations from postgres")
}

pub fn get_secret_key_by_origin(data_store: originsrv_data_store,
                                origin_name: &str)
                                -> Option<protocol::originsrv::OriginSecretKey> {
    let mut oskg = protocol::originsrv::OriginSecretKeyGet::new();
    oskg.set_origin(origin_name.to_string());
    data_store
        .get_origin_secret_key(&oskg)
        .expect("unable to get secret keys")
}

pub fn create_test_originsrv_data_store() -> originsrv_data_store {
    let ds = datastore_test!(originsrv_data_store);
    ds
}

pub fn create_test_sessionsrv_data_store() -> sessionsrv_data_store {
    let ds = datastore_test!(sessionsrv_data_store);
    ds
}

pub fn create_sessionsrv_data_store() -> sessionsrv_data_store {
    let sessionsrv_config = data_store_config("builder_sessionsrv");

    let pool = create_pool(sessionsrv_config);

    let sessionsrv_data_store = sessionsrv_data_store { pool: pool };
    sessionsrv_data_store
}

pub fn create_originsrv_data_store() -> originsrv_data_store {
    let originsrv_config = data_store_config("builder_originsrv");
    let pool = create_pool(originsrv_config);

    let ap = pool.clone();

    let originsrv_data_store = originsrv_data_store {
        pool: pool,
        async: hab_db::async::AsyncServer::new(ap),
    };
    originsrv_data_store
}

pub fn get_origin_keys_by_origin(data_store: originsrv_data_store,
                                 origin_id: u64)
                                 -> protocol::originsrv::OriginPublicKeyListResponse {
    let mut request = protocol::originsrv::OriginPublicKeyListRequest::new();
    request.set_origin_id(origin_id);
    let keys = data_store.list_origin_public_keys_for_origin(&request);
    keys.unwrap()
}

pub fn get_origin_key_by_revision(data_store: originsrv_data_store,
                                  origin: &str,
                                  revision: &str)
                                  -> Option<protocol::originsrv::OriginPublicKey> {
    let mut request = protocol::originsrv::OriginPublicKeyGet::new();
    request.set_origin(origin.to_string());
    request.set_revision(revision.to_string());
    data_store
        .get_origin_public_key(&request)
        .expect("failed to get public keys")
}

pub fn is_account_in_origin(data_store: sessionsrv_data_store,
                            origin: String,
                            account_id: u64)
                            -> bool {
    let mut request = protocol::sessionsrv::AccountOriginListRequest::new();
    request.set_account_id(account_id);
    for o in data_store
            .get_origins_by_account(&request)
            .expect("failed to list origins by account")
            .get_origins() {
        if o.as_str() == origin {
            return true;
        }
    }
    return false;
}

pub fn is_member_in_origin(data_store: originsrv_data_store,
                            account: String,
                            origin_id: u64)
                            -> bool {
    let mut request = protocol::originsrv::OriginMemberListRequest::new();
    request.set_origin_id(origin_id);
    for m in data_store
            .list_origin_members(&request)
            .expect("failed to list origins members")
            .get_members() {
        if m.as_str() == account {
            return true;
        }
    }
    return false;
}

pub fn create_account_origin(data_store: sessionsrv_data_store,
                            origin_id: u64,
                            origin_name: &str,
                            account_id: u64,
                            account_name: &str) {
    let mut aoc = protocol::sessionsrv::AccountOriginCreate::new();
    aoc.set_account_id(account_id);
    aoc.set_origin_id(origin_id);
    aoc.set_origin_name(origin_name.to_string());
    aoc.set_account_name(account_name.to_string());
    data_store.create_origin(&aoc).expect("unable to save account origin");
}

fn data_store_config(database_name: &str) -> hab_db::config::DataStoreCfg {
    let pw = match env::var("PGPASSWORD") {
        Ok(password) => Some(password.to_string()),
        Err(_) => None,
    };

    let config = hab_db::config::DataStoreCfg {
        host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 5432,
        user: String::from("hab"),
        password: pw,
        database: String::from(database_name),
        connection_retry_ms: 300,
        connection_timeout_sec: 60,
        connection_test: false,
        pool_size: (num_cpus::get() * 2) as u32,
    };

    config
}

fn create_pool_config_builder
    (config: hab_db::config::DataStoreCfg)
     -> r2d2::config::Builder<postgres::Connection, r2d2_postgres::Error> {
    let pool_builder = r2d2::Config::<postgres::Connection, r2d2_postgres::Error>::builder()
        .pool_size(config.pool_size)
        .connection_timeout(Duration::from_secs(config.connection_timeout_sec));
    pool_builder
}

fn r2d2_pool(config: hab_db::config::DataStoreCfg)
             -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager> {
    let pool_config_builder = create_pool_config_builder(config.clone());
    let pool_config = pool_config_builder.build();

    let manager = PostgresConnectionManager::new(&config, TlsMode::None).unwrap();
    let r2d2_pool = r2d2::Pool::new(pool_config, manager).unwrap();
    r2d2_pool
}

fn create_pool(config: hab_db::config::DataStoreCfg) -> hab_db::pool::Pool {
    let mut shards: Vec<protocol::sharding::ShardId> = (1..128).collect();

    let r2d2_pool = r2d2_pool(config);

    let pool = hab_db::pool::Pool {
        inner: r2d2_pool,
        shards: shards,
    };
    pool
}
