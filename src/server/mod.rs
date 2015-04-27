use std::collections::HashMap;
use std::io::Write;
use std::cell::{Cell, RefCell};
use core;

use uidgen::TS6UIDGenerator;
use message::Message;

mod command;

pub type Token = usize;

pub struct Client {
    token: Token,
    nickname: RefCell<Option<String>>,
    username: RefCell<Option<String>>,
    realname: RefCell<Option<String>>,
    hostname: RefCell<Option<String>>,
    out_socket: RefCell<Box<Write>>,
}

impl Client {
    pub fn new(token: Token, out_socket: Box<Write>, hostname: String) -> Client {
        Client {
            token: token,
            nickname: RefCell::new(None),
            username: RefCell::new(None),
            realname: RefCell::new(None),
            hostname: RefCell::new(Some(hostname)),
            out_socket: RefCell::new(out_socket),
        }
    }

    pub fn nickname(&self) -> Option<String> { self.nickname.borrow().clone() }
    pub fn set_nickname(&self, new: String) { *self.nickname.borrow_mut() = Some(new); }

    pub fn username(&self) -> Option<String> { self.username.borrow().clone() }
    pub fn set_username(&self, new: String) { *self.username.borrow_mut() = Some(new); }

    pub fn realname(&self) -> Option<String> { self.realname.borrow().clone() }
    pub fn set_realname(&self, new: String) { *self.realname.borrow_mut() = Some(new); }

    pub fn registered(&self) -> bool {
        self.username.borrow().is_some() && self.nickname.borrow().is_some()
    }

    pub fn prefix(&self) -> String {
        format!("{}!{}@{}",
            self.nickname.borrow().as_ref().unwrap(),
            self.username.borrow().as_ref().unwrap(),
            self.hostname.borrow().as_ref().unwrap())
    }
}

impl core::fmt::Debug for Client {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "<Client>")
    }
}

pub struct Server {
    name: String,
    display_name: String,
    sid: String,
    clients_tok: HashMap<Token, Client>,
    nicknames: RefCell<HashMap<String, Token>>,
    uuidgen: RefCell<TS6UIDGenerator>,
}

impl Server {
    pub fn new(name: String, display_name: String, sid: String) -> Server {
        let uuidgen = TS6UIDGenerator::new(sid.as_str());
        Server {
            name: name,
            display_name: display_name,
            sid: sid,
            clients_tok: HashMap::new(),
            nicknames: RefCell::new(HashMap::new()),
            uuidgen: RefCell::new(uuidgen),
        }
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn accept_connection(&mut self, out_socket: Box<Write>, token: Token, hostname: String) {
        let client = Client::new(token, out_socket, hostname);

        self.clients_tok.insert(token, client);
    }

    pub fn process_line(&mut self, token: Token, line: &str) {
        if let Some(message) = Message::new(line) {
            self.resolve_command(message, self.client_by_token(token).unwrap());
        } else {
            // TODO: received garbage
        }
    }

    pub fn client_by_token(&self, token: Token) -> Option<&Client> {
        self.clients_tok.get(&token)
    }

    pub fn client_by_nickname(&self, nickname: &String) -> Option<&Client> {
        let tok = self.nicknames.borrow().get(nickname).unwrap().clone();
        self.client_by_token(tok)
    }

    fn register_client(&self, client: &Client) {
        let nickname = client.nickname().unwrap();
        self.nicknames.borrow_mut().insert(nickname.clone(), client.token);
        write!(client.out_socket.borrow_mut(),
            ":{} 001 {} :Welcome to the {} Internet Relay Chat Network {}\r\n",
            self.name, nickname, self.display_name, nickname);
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::io;
    use std::cell::RefCell;

    pub use server::*;
    pub use hamcrest::{assert_that, is, not, none, equal_to};

    pub struct TestSock { buf: Rc<RefCell<Vec<u8>>> }

    impl TestSock {
        pub fn new() -> TestSock {
            TestSock {
                buf: Rc::new(RefCell::new(Vec::new()))
            }
        }

        pub fn data(&self) -> String {
            String::from_utf8(self.buf.borrow().clone()).ok().unwrap()
        }

        pub fn clear(&self) {
            self.buf.borrow_mut().clear();
        }
    }

    impl io::Write for TestSock {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.borrow_mut().write(buf)
        }
        fn flush(&mut self) -> io::Result<()> {
            self.buf.borrow_mut().flush()
        }
    }

    impl Clone for TestSock {
        fn clone(&self) -> Self {
            TestSock {
                buf: self.buf.clone()
            }
        }
    }

    describe! accepted_connection {
        before_each {
            let mut server = Server::new("test.local".to_string(), "TestLocal".to_string(),"42X".to_string());

            let sock = Box::new(TestSock::new());
            let token = 8;

            server.accept_connection(sock.clone(), token, "127.0.0.1".to_string());
        }

        it "is resolved by token" {
            assert_that(server.client_by_token(8), is(not(none())));
        }

        it "registers user upon NICK/USER pair sent" {
            server.process_line(token, "NICK test");
            server.process_line(token, "USER guest 0 * :Ronnie Reagan");

            let user = server.client_by_token(8).unwrap();

            assert_that(user.registered(), is(equal_to(true)));
        }

        it "sends welcome to registered user when USER/NICK is sent" {
            server.process_line(token, "USER guest 0 * :Ronnie Reagan");
            server.process_line(token, "NICK test");

            let outbuf = sock.data();
            assert_that(outbuf.as_str(),
                is(equal_to(":test.local 001 test :Welcome to the TestLocal Internet Relay Chat Network test\r\n")));
        }

        it "sends welcome to registered user when NICK/USER is sent" {
            server.process_line(token, "NICK test");
            server.process_line(token, "USER guest 0 * :Ronnie Reagan");

            assert_that(sock.data().as_str(),
                is(equal_to(":test.local 001 test :Welcome to the TestLocal Internet Relay Chat Network test\r\n")));
        }
    }
}
