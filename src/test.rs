use account;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod AccountsDB {
    use super::*;

    #[test]
    fn create_works() {
        let db = account::AccountsDB::create();
        println!("{:?}", db);
    }

    #[test]
    fn import_works() {
        let mut db = account::AccountsDB::create();
        db.import("test.json");
        println!("{:?}", db);
    }

    #[test]
    fn export_works() {
        let mut db = account::AccountsDB::create();
        db.import("test.json");
        db.export("test.json");
    }

    #[test]
    fn query_found() {
        let mut db = account::AccountsDB::create();
        db.import("test.json");
        println!("{:?}", db.query("username").unwrap());
    }

    #[test]
    fn query_notfound() {
        let mut db = account::AccountsDB::create();
        db.import("test.json");
        println!("{:?}", db.query("notfound"));
    }

    #[test]
    fn authorize_right_account() {
        let mut db = account::AccountsDB::create();
        db.import("test.json");
        println!("{:?}", db.authorize("username", "password").unwrap());
        assert_eq!(db.authorize("username", "password").unwrap(), true);
    }

    #[test]
    fn authorize_wrong_account() {
        let mut db = account::AccountsDB::create();
        db.import("test.json");
        println!("{:?}", db.authorize("username", "wrong_password").unwrap());
        assert_eq!(db.authorize("username", "wrong_password").unwrap(), false);
    }

    #[test]
    fn authorize_notfound() {
        let mut db = account::AccountsDB::create();
        db.import("test.json");
        println!("{:?}", db.authorize("notfound", "password"));
    }

    #[test]
    fn register_success() {
        let mut db = account::AccountsDB::create();
        db.import("test.json");
        println!("{:?}", db.register("new_account", "new_password"));
        println!("{:?}", db.query("new_account"));
        assert_eq!(db.exists("new_account"), true);
    }
}
