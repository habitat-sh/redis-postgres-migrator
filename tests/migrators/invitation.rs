use redis_postgres_migrator::migrators;
use postgres_lib;
use redis_lib;

const TEST_REDIS_ADDR: &'static str = "redis://127.0.0.1:6379";

#[test]
fn test_migrate_invitations() {
    let ds = postgres_lib::create_test_originsrv_data_store();
    ds.setup();
    let sds = postgres_lib::create_test_sessionsrv_data_store();
    sds.setup();
    
    let redis_session_inviter = redis_lib::create_session(String::from("token1"),
                                                          64,
                                                          String::from("bobo@chef.io"),
                                                          String::from("inviter"));
    let redis_account_inviter = redis_lib::create_account(TEST_REDIS_ADDR, redis_session_inviter);
    let pg_session_inviter = postgres_lib::create_session(String::from("token1"),
                                                          64,
                                                          String::from("bobo@chef.io"),
                                                          String::from("inviter"));
    let pg_account_inviter = postgres_lib::create_account(sds.clone(), pg_session_inviter);
    let pg_session_invitee1 = postgres_lib::create_session(String::from("token2"),
                                                           64,
                                                           String::from("booboo@chef.io"),
                                                           String::from("invitee1"));
    let pg_account_invitee1 = postgres_lib::create_account(sds.clone(), pg_session_invitee1);
    let pg_session_invitee2 = postgres_lib::create_session(String::from("token3"),
                                                           64,
                                                           String::from("beebee@chef.io"),
                                                           String::from("invitee2"));
    let pg_account_invitee2 = postgres_lib::create_account(sds.clone(), pg_session_invitee2);

    let redis_origin =
        redis_lib::create_origin(TEST_REDIS_ADDR, "blah", redis_account_inviter.get_id());
    let pg_origin = postgres_lib::create_origin(ds.clone(),
                                                redis_origin.get_name(),
                                                redis_account_inviter.get_id(),
                                                redis_account_inviter.get_name())
            .unwrap();

    let invitation1 = redis_lib::create_invitation(TEST_REDIS_ADDR,
                                                   5000,
                                                   pg_account_invitee1.get_name(),
                                                   redis_origin.get_id(),
                                                   redis_origin.get_name(),
                                                   redis_account_inviter.get_id());
    let invitation2 = redis_lib::create_invitation(TEST_REDIS_ADDR,
                                                   5001,
                                                   pg_account_invitee2.get_name(),
                                                   redis_origin.get_id(),
                                                   redis_origin.get_name(),
                                                   redis_account_inviter.get_id());
    let invitation3 = redis_lib::create_invitation(TEST_REDIS_ADDR,
                                                   5001,
                                                   pg_account_invitee2.get_name(),
                                                   redis_origin.get_id(),
                                                   redis_origin.get_name(),
                                                   redis_account_inviter.get_id());

    migrators::invitation::InvitationMigrator::new(TEST_REDIS_ADDR.to_string(),
                                                   ds.clone(),
                                                   sds.clone())
            .migrate();

    let pg_invitations = postgres_lib::get_invitations_by_origin(ds.clone(), pg_origin.get_id());

    assert_eq!(2, pg_invitations.get_invitations().len());

    assert_eq!(pg_account_invitee1.get_id(),
               pg_invitations.get_invitations()[0].get_account_id());
    assert_eq!(pg_account_invitee1.get_name(),
               pg_invitations.get_invitations()[0].get_account_name());
    assert_eq!(pg_origin.get_id(),
               pg_invitations.get_invitations()[0].get_origin_id());
    assert_eq!(pg_origin.get_name(),
               pg_invitations.get_invitations()[0].get_origin_name());
    assert_eq!(pg_account_inviter.get_id(),
               pg_invitations.get_invitations()[0].get_owner_id());

    assert_eq!(pg_account_invitee2.get_id(),
               pg_invitations.get_invitations()[1].get_account_id());
    assert_eq!(pg_account_invitee2.get_name(),
               pg_invitations.get_invitations()[1].get_account_name());
    assert_eq!(pg_origin.get_id(),
               pg_invitations.get_invitations()[1].get_origin_id());
    assert_eq!(pg_origin.get_name(),
               pg_invitations.get_invitations()[1].get_origin_name());
    assert_eq!(pg_account_inviter.get_id(),
               pg_invitations.get_invitations()[1].get_owner_id());
}
