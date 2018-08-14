extern crate serde_json;

use self::serde_json::Value;
use account;
use client;
use std::net::UdpSocket;
use std::str;

#[derive(Debug)]
enum Pattern {
    Login(account::AccountInfo),
    Register(account::AccountInfo),
    Exit(String),
    Comment(client::CommentInfo),
}

#[derive(Serialize, Deserialize, Debug)]
struct Request<'a> {
    command: &'a str,
    args: Value,
}

#[derive(Debug)]
pub struct MyChannel {
    pub name: String,
    // ssl context needed
    listener: UdpSocket,
    sessions: Vec<String>,
    account_system: &'static account::AccountsDB,
    pub max_users: usize,
}

impl MyChannel {
    pub fn new(
        name: String,
        addr: &'static str,
        max_users: usize,
        account_system: &'static account::AccountsDB,
    ) -> MyChannel {
        MyChannel {
            name,
            listener: UdpSocket::bind(addr).unwrap(),
            sessions: vec![],
            account_system,
            max_users,
        }
    }
    pub fn run(&self) {
        let mut i = 0;
        let mut buf = [0; 1024];
        loop {
            let (amt, src) = self.listener.recv_from(&mut buf).unwrap();
            let msg = str::from_utf8(&buf[..amt]).unwrap();
            let msg: Request = serde_json::from_str(&msg).unwrap();
            println!("{:?}", msg);
            let patt = deliver(msg);
            println!("{:?}", patt);
            self.deal(patt);
        }
    }
    fn deal(&self, patt: Pattern) {}
}

fn deliver<'a>(msg: Request<'a>) -> Pattern {
    match msg.command {
        "Login" => {
            let info: account::AccountInfo = serde_json::from_value(msg.args).unwrap();
            Pattern::Login(info)
        }
        "Register" => {
            let info: account::AccountInfo = serde_json::from_value(msg.args).unwrap();
            Pattern::Register(info)
        }
        "Exit" => {
            let id = msg.args["name"].clone();
            Pattern::Exit(id.to_string())
        }
        "Comment" => {
            let comment: client::CommentInfo = serde_json::from_value(msg.args).unwrap();
            Pattern::Comment(comment)
        }
        _ => panic!("Invalid JSON format"),
    }
}
