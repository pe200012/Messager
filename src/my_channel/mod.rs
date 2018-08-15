extern crate serde_json;

use my_channel::serde_json::Value::Null;
use self::serde_json::Value;
use account;
use client;
use std::net::SocketAddr;
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

#[derive(Serialize, Deserialize, Debug)]
struct Response<'a> {
    command: &'a str,
    args: Value,
}

#[derive(Debug)]
pub struct MyChannel {
    pub name: String,
    // ssl context needed
    listener: UdpSocket,
    sessions: Vec<SocketAddr>,
    account_system: &'static mut account::AccountsDB,
    pub max_users: usize,
}

impl MyChannel {
    pub fn new(
        name: String,
        addr: &'static str,
        max_users: usize,
        account_system: &'static mut account::AccountsDB,
    ) -> MyChannel {
        MyChannel {
            name,
            listener: UdpSocket::bind(addr).unwrap(),
            sessions: vec![],
            account_system,
            max_users,
        }
    }
    pub fn run(&'static mut self) {
        let mut i = 0;
        let mut buf = [0; 1024];
        loop {
            let (amt, src) = self.listener.recv_from(&mut buf).unwrap();
            let msg = str::from_utf8(&buf[..amt]).unwrap();
            let msg: Request = serde_json::from_str(&msg).unwrap();
            println!("{:?} from {:?}", msg, src);
            self.sessions.push(src);
            let patt = deliver(msg);
            println!("{:?}", patt);
            self.deal(patt, src);
            i = i + 1;
        }
    }
    fn deal(&mut self, patt: Pattern, remote: SocketAddr) {
        println!("Pattern: {:?}", patt);
        let res : Response = match patt {
            Pattern::Login(info) => {
                match self.account_system.authorize(&info.name, &info.password) {
                    Err(_) => {
                        Response {
                            command: "AccountNotFound",
                            args: Null,
                        }
                    },
                    Ok(false) => {
                        Response {
                            command: "WrongPassword",
                            args: Null,
                        }
                    },
                    Ok(true) => {
                        Response {
                            command: "LoginSuccess",
                            args: Null,
                        }
                    },
                }
            }
            Pattern::Register(info) => {
                match self.account_system.register(&info.name, &info.password) {
                    false => {
                        Response {
                            command: "AccountAlreadyExists",
                            args: Null,
                        }
                    },
                    true => {
                        Response {
                            command: "RegisterSuccess",
                            args: Null,
                        }
                    }
                }
            }
            Pattern::Comment(comment) => {
                self.broadcast(comment);
                Response {
                    command: "CommentSuccess",
                    args: Null,
                }
            },
            Pattern::Exit(id) => {
                // Nightly feature
                // self.sessions.remove_item(&remote).unwrap();
                for (index, session) in self.sessions.iter().enumerate() {
                    if *session == remote {
                        self.sessions.remove(index);
                        break;
                    }
                }
                Response {
                    command: "ExitSuccess",
                    args: Null,
                }
            },
        };
    }
    fn broadcast(&self, comment: client::CommentInfo) {
        for session in self.sessions.iter().enumerate() {
            match self.listener.send_to(b"", session) {
                _ => ()
            }
        }
    }
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
