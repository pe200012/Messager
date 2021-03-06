extern crate serde_json;

use self::serde_json::Value;
use account;
use client;
use my_channel::serde_json::Value::Null;
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
    result: &'a str,
    args: Value,
}

#[derive(Debug)]
pub struct MyChannel<'a> {
    pub name: String,
    // ssl context needed
    listener: UdpSocket,
    sessions: Vec<SocketAddr>,
    account_system: &'a mut account::AccountsDB,
    pub max_users: usize,
}

impl<'a> MyChannel<'a> {
    pub fn new(
        name: String,
        addr: &'static str,
        max_users: usize,
        account_system: &'a mut account::AccountsDB,
    ) -> MyChannel<'a> {
        MyChannel {
            name,
            listener: UdpSocket::bind(addr).unwrap(),
            sessions: vec![],
            account_system,
            max_users,
        }
    }
    pub fn run(&'a mut self) {
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
        let res: Response = match patt {
            Pattern::Login(info) => match self.account_system.authorize(&info.name, &info.password)
            {
                Err(_) => Response {
                    result: "AccountNotFound",
                    args: Null,
                },
                Ok(false) => Response {
                    result: "WrongPassword",
                    args: Null,
                },
                Ok(true) => Response {
                    result: "LoginSuccess",
                    args: Null,
                },
            },
            Pattern::Register(info) => {
                match self.account_system.register(&info.name, &info.password) {
                    false => Response {
                        result: "AccountAlreadyExists",
                        args: Null,
                    },
                    true => Response {
                        result: "RegisterSuccess",
                        args: Null,
                    },
                }
            }
            Pattern::Comment(comment) => {
                self.broadcast(comment);
                Response {
                    result: "CommentSuccess",
                    args: Null,
                }
            }
            Pattern::Exit(id) => {
                self.sessions.remove_item(&remote).unwrap();
                Response {
                    result: "ExitSuccess",
                    args: Null,
                }
            }
        };
        self.listener
            .send_to(serde_json::to_string(&res).unwrap().as_bytes(), remote)
            .unwrap();
    }
    fn broadcast(&self, comment: client::CommentInfo) {
        for (_, session) in self.sessions.iter().enumerate() {
            match self
                .listener
                .send_to(serde_json::to_string(&comment).unwrap().as_bytes(), session)
            {
                _ => (),
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
