extern crate crypto;
extern crate habitat_depot as depot;
extern crate habitat_builder_sessionsrv_redis as hab_sessionsrv;
extern crate habitat_builder_protocol_redis as protocol;
extern crate habitat_builder_dbcache_redis as dbcache;
extern crate habitat_builder_vault as vault;
extern crate habitat_core_redis as hab_core;
extern crate r2d2;
extern crate r2d2_redis;

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::env;
use std::net;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use dbcache::BasicSet;
use dbcache::data_store::Pool;
use dbcache::IndexSet;
use dbcache::InstaSet;
use depot::data_store::DataStore as depot_datastore;
use vault::data_store::DataStore as vault_datastore;
use hab_sessionsrv::data_store::{DataStore, AccountTable};
use hab_core::package::{FromArchive, PackageArchive};

use std::fs::{self, File};
use std::io::{Read, Write, BufWriter};
use std::io::prelude::*;

use self::r2d2_redis::RedisConnectionManager;
use std::str::FromStr;

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

pub fn create_account(redis_addr: &str,
                      session: protocol::sessionsrv::SessionCreate)
                      -> protocol::sessionsrv::Account {

    let pool = create_pool(redis_addr);
    let account_table = hab_sessionsrv::data_store::AccountTable::new(pool);

    let mut account = protocol::sessionsrv::Account::new();
    account.set_email(session.get_email().to_string());
    account.set_name(session.get_name().to_string());
    account_table.write(&mut account);

    account
}

pub fn create_origin(redis_addr: &str, name: &str, owner_id: u64) -> protocol::vault::Origin {
    let mut origin = protocol::vault::Origin::new();
    origin.set_owner_id(owner_id);
    origin.set_name(name.to_string());
    let datastore = vault_datastore::init(create_pool(redis_addr));
    datastore.origins.write(&mut origin);
    origin
}

pub fn create_invitation(redis_addr: &str,
                         account_id: u64,
                         account_name: &str,
                         origin_id: u64,
                         origin_name: &str,
                         owner_id: u64)
                         -> protocol::vault::OriginInvitation {
    let mut create = protocol::vault::OriginInvitation::new();
    create.set_account_id(account_id);
    create.set_account_name(account_name.to_string());
    create.set_origin_id(origin_id);
    create.set_origin_name(origin_name.to_string());
    create.set_owner_id(owner_id);
    let datastore = vault_datastore::init(create_pool(redis_addr));
    datastore.origins.invites.write(&mut create);
    create
}

pub fn get_invitations_by_origin(redis_addr: &str,
                                 origin_id: u64)
                                 -> Vec<protocol::vault::OriginInvitation> {
    let ds = vault_datastore::init(create_pool(redis_addr));
    ds.origins
        .invites
        .get_by_origin_id(origin_id)
        .expect("unable to list invitations for origin")
}

pub fn get_secret_key_by_id(redis_addr: &str,
                                id: u64)
                                -> protocol::vault::OriginSecretKey {
    let ds = vault_datastore::init(create_pool(redis_addr));
    ds.origins.origin_secret_keys.find(&id).unwrap()
}

pub fn create_package(redis_addr: &str, hart: PathBuf) -> protocol::depotsrv::Package {
    let mut archive = PackageArchive::new(hart);
    let package = protocol::depotsrv::Package::from_archive(&mut archive).expect("unable to create package from archive");
    create_depot_datastore(redis_addr)
        .packages
        .write(&package)
        .expect("unable to save package to redis");
    package
}

pub fn find_account_by_id(redis_addr: &str, id: String) -> protocol::sessionsrv::Account {
    let pool = create_pool(redis_addr);
    let ds = DataStore::new(pool);

    let value = account_value(id);
    let account = ds.accounts.find(&value).unwrap();

    account
}

pub fn find_origin_by_id(redis_addr: &str, id: u64) -> protocol::vault::Origin {
    let ds = vault_datastore::init(create_pool(redis_addr));
    ds.origins.find(&id).unwrap()
}

pub fn create_pool(redis_addr: &str)
                   -> std::sync::Arc<r2d2::Pool<r2d2_redis::RedisConnectionManager>> {
    let config = Default::default();
    let manager = RedisConnectionManager::new(redis_addr).unwrap();
    let mut pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());
    pool
}

pub fn get_package_idents_by_origin(redis_addr: &str,
                                    origin: &str)
                                    -> Vec<protocol::depotsrv::PackageIdent> {
    create_depot_datastore(redis_addr)
        .packages
        .index
        .list(origin, 0, -1)
        .expect("unable to get packages from origin")
}

pub fn get_package_by_ident(redis_addr: &str,
                            ident: protocol::depotsrv::PackageIdent)
                            -> protocol::depotsrv::Package {
    create_depot_datastore(redis_addr)
        .packages
        .find(&ident)
        .expect("unable to get package from redis")
}

pub fn create_origin_key(origin: &str, revision: &str, key_content: String, redis_addr: &str) {
    let mut depot_config = depot::Config::default();
    depot_config.datastore_addr = net::SocketAddrV4::from_str(redis_addr
                                                                  .replace("redis://", "")
                                                                  .as_str())
            .expect("bad address");
    let depot = depot::Depot::new(depot_config).unwrap();

    let origin_key_file = depot.key_path(origin, revision);

    write_string_to_file(&origin_key_file, key_content);

    depot.datastore.origin_keys.write(&origin, &revision);
}

pub fn create_secret_key(origin_id: u64,
                         name: &str,
                         revision: &str,
                         body: &str,
                         owner_id: u64,
                         redis_addr: &str)
                         -> protocol::vault::OriginSecretKey {
    let mut key = protocol::vault::OriginSecretKey::new();
    key.set_owner_id(owner_id);
    key.set_name(name.to_string());
    key.set_body(body.as_bytes().to_vec());
    key.set_origin_id(origin_id);
    key.set_revision(revision.to_string());
    let datastore = vault_datastore::init(create_pool(redis_addr));
    datastore.origins.origin_secret_keys.write(&mut key);
    key
}

pub fn get_origin_keys_by_origin(origin: &str,
                                 redis_addr: &str)
                                 -> Vec<protocol::depotsrv::OriginKeyIdent> {
    let mut depot_config = depot::Config::default();
    depot_config.datastore_addr = net::SocketAddrV4::from_str(redis_addr
                                                                  .replace("redis://", "")
                                                                  .as_str())
            .expect("bad address");
    let depot = depot::Depot::new(depot_config).unwrap();
    let keys_list = depot.datastore.origin_keys.all(origin);

    keys_list.unwrap()
}

pub fn get_key_body(origin: &str, revision: &str) -> String {
    let location = key_path(origin, revision);
    let mut file = match File::open(location) {
        Ok(file) => file,
        Err(_) => panic!("that key file does not exist"),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .ok()
        .expect("failed to read that key file");

    contents
}

fn key_path(key: &str, rev: &str) -> PathBuf {
    let config_path = match env::var("KEY_ROOT") {
        Ok(root) => root.to_string(),
        Err(_) => String::from("/hab/svc/builder-depot/data")
    };

    let mut digest = Sha256::new();
    let mut output = [0; 64];
    let key_with_rev = format!("{}-{}.pub", key, rev);
    digest.input_str(&key_with_rev.to_string());
    digest.result(&mut output);
    Path::new(&config_path).join("keys")
        .join(format!("{:x}", output[0]))
        .join(format!("{:x}", output[1]))
        .join(format!("{}-{}.pub", key, rev))
}

fn write_string_to_file(filename: &PathBuf, body: String) -> Result<bool, depot::Error> {
    let path = filename.parent().unwrap();
    try!(fs::create_dir_all(path));
    let tempfile = format!("{}.tmp", filename.to_string_lossy());
    let f = try!(File::create(&tempfile));
    let mut writer = BufWriter::new(&f);
    try!(writer.write_all(body.as_bytes()));
    //    info!("File added to Depot at {}", filename.to_string_lossy());
    try!(fs::rename(&tempfile, &filename));
    Ok(true)
}

fn create_depot_datastore(redis_addr: &str) -> depot_datastore {
    let mut config = depot::Config::default();
    config.datastore_addr = net::SocketAddrV4::from_str(redis_addr
                                                            .replace("redis://", "")
                                                            .as_str())
            .expect("bad address");
    depot_datastore::open(&config).unwrap()
}

fn account_value(id: String) -> u64 {
    let account_search_key = protocol::sessionsrv::AccountSearchKey::Id;
    let mut account_search = protocol::sessionsrv::AccountSearch::new();

    account_search.set_key(account_search_key);
    account_search.set_value(id.clone());

    let value: u64 = account_search.take_value().parse().unwrap();
    value
}
