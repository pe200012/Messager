extern crate serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountInfo {
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountsPresent {
    accounts: Vec<AccountInfo>,
}

#[derive(Debug)]
pub struct AccountsDB {
    data: AccountsPresent,
}

impl AccountsDB {
    pub fn create() -> AccountsDB {
        AccountsDB {
            data: AccountsPresent { accounts: vec![] },
        }
    }
    pub fn import(&mut self, file: &'static str) {
        let mut f = File::open(file).unwrap();
        let mut buf = vec![];
        f.read_to_end(&mut buf).unwrap();
        let buf = str::from_utf8(&buf).unwrap();
        let ap: AccountsPresent = serde_json::from_str(&buf).unwrap();
        self.data = ap;
    }
    pub fn export(&mut self, file: &'static str) -> Result<usize, Error> {
        let mut f = File::create(file).unwrap();
        let contents = serde_json::to_string(&self.data).unwrap();
        let contents = contents.as_bytes();
        f.write(&contents)
    }
    pub fn exists(&self, id: &'static str) -> bool {
        for (_, account) in self.data.accounts.iter().enumerate() {
            if account.name == *id {
                return true;
            }
        }
        false
    }
    pub fn query(&self, id: &'static str) -> Option<AccountInfo> {
        for (_, account) in self.data.accounts.iter().enumerate() {
            if account.name == *id {
                return Some(account.clone());
            }
        }
        None
    }
    pub fn authorize(&self, id: &'static str, password: &'static str) -> Result<bool, Error> {
        match self.query(id) {
            None => return Err(Error::new(ErrorKind::NotFound, "User not found")),
            Some(a) => {
                if a.password == password {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
        }
    }
    pub fn register(&mut self, id: &'static str, password: &'static str) -> bool {
        match self.query(id) {
            Some(_) => false,
            None => {
                let new_account = AccountInfo {
                    name: id.to_string(),
                    password: password.to_string(),
                };
                self.data.accounts.push(new_account);
                true
            }
        }
    }
}
