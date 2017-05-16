# redis-postgres-migrator

# How do I run this?

This is assuming that you have a running Habitat instance of at least 0.22.1.  This also assumes you have copyied a redis store to your new Habitat instance and are running it.

You will need to clone this repo to your new Habitat instance then run 

```
$ cargo run -- -- "redis://127.0.0.1:6379" 
```

# What is this?

This is a tool for migrating data from a redis data store (which a system running Habitat 0.12.x would have) to a postgres data store (which a system running the latest release of Habitat would have).

Data is stored in a serialized format in both Habitat 0.12.x and the latest version of Habitat.  This presents a challenge - both the serialization and schemas are different between these two version of Habitat, it will not be a 1:1 migration.

How I'm addressing this is by using the data store code from Habitat 0.12.1 to extract and deserialize the data stored in the redis data store, then using data store code from the latest release of Habitat to reserialize that data and insert it into a postgres store.

# How does it work?

This tool is a binary cargo project.  It uses 3 internal crates.

## Why did you use crates?  Why didn't you just use modules?

I started out using modules.  The problem came from the fact that the separate modules needed to use different versions of the same dependencies.  This caused A LOT of conflict when attempting to resolve dependencies.  So I decided to construct them as self-contained crates, where the dependency versions wouldn't cross over or be confused between them.

## redis_extraction crate

The sole purpose of this crate is to extract a list of account ids from the redis data store.  Remember, the data in redis is serialized, but the keys for each account include the account id.  By pulling out the list of KEYS for accounts in the redis data store, I can then parse out the account id from each of them.  Everything in the data migration depends on having the account id.

## redis_lib

This crate contains all the Habitat 0.12.1 code required to deserialize the data from the redis store.

## postgres_lib

This crate contains all the Habitat master code (master as of 5/2/17) that can serialize the data extracted by redis_lib and insert it into the postgres store.


# Can you give me an example of how this works?

Certainly!

Lets say we have a redis data store (if you want a copy of the current Habitat production redis store, ping Nell Shamrell-Harrington).  That data store will include keys in this format "account:123".  The 123 is the account id.

Most of the functionality for this program lives in the redis_to_postgres function of the main.rs file.  This method takes the address of a redis data store, then the postgres data store as an object (this may be changed in the future).

```
pub fn redis_to_postgres(redis_addr: &str, data_store: session_srv::data_store::DataStore)
```

This function then will pull the list of redis account keys from the redis store

```
let accounts = redis_extraction::extract_accounts(redis_addr);
```

Which will return a vector consisting of strings that look like this "acount:123").  We then iterate over each of these strings and use a regex to capture the account number (we use a capture group to do this).

```
let re = Regex::new(r":(\d+)").unwrap();
for x in accounts {
		for cap in re.captures_iter(&x) {
				let ds = data_store.clone();
				let account_id_string = &cap[1];
				let account_id = account_id_string.parse::<u64>();
```

Next, we're going to actually insert that data into postgres.  We pass the address of the data store, the postgres data store, and the account id to another function.

```
redis_to_postgres_account(redis_addr,
													ds,
													account.get_id())
```


```
pub fn redis_to_postgres_account(redis_addr: &str,
                                 data_store: session_srv::data_store::DataStore,
                                 id: u64) {
    let redis_account = redis_lib::find_account_by_id(redis_addr, id.to_string());
    let config = session_srv::config::Config::default();

    let session = postgres_lib::create_session("pretend_session".to_string(),
                                               redis_account.get_id(),
                                               redis_account.get_email().to_string(),
                                               redis_account.get_name().to_string());

    let account = postgres_lib::create_account(data_store, session);
}
```

This method finds the redis account, then creates a session with the postgres_lib (this is the latest Hab release code), then creates that actual account in the postgres data store.

# So what's next?

We need to import several of the key value pairs from redis and insert them with the correct serialization in the correct places in the postgres datastore.  Check out [this issue](https://github.com/habitat-sh/habitat/issues/1935) for more information.
