use server::{Server, Client};
use message::Message;

impl Server {
    pub fn cmd_privmsg(&self, message: Message, client: &Client) {
        let target = message.arguments[0];
        let text = message.arguments[1];

        let target_client = self.client_by_nickname(&target.to_string()).unwrap();
        let src_prefix = client.prefix();
        let msg = Message::build(Some(src_prefix.as_str()), "PRIVMSG", vec!(target, text));

        write!(target_client.out_socket.borrow_mut(), "{:?}\r\n", msg);
    }
}

#[cfg(test)]
mod test {
    pub use server::test::TestSock;
    pub use server::*;
    pub use hamcrest::{assert_that, is, not, none, equal_to};

    describe! accepted_connection {
        before_each {
            let mut server = Server::new("test.local".to_string(), "TestLocal".to_string(),"42X".to_string());

            let sock = Box::new(TestSock::new());
            let token = 8;

            server.accept_connection(sock.clone(), token, "127.0.0.1".to_string());
        }

        describe! registered_client {
            before_each {
                server.process_line(token, "NICK test");
                server.process_line(token, "USER guest 0 * :Ronnie Reagan");
                sock.clear();
            }

            it "can message itself" {
                server.process_line(token, "PRIVMSG test test");

                assert_that(sock.data().as_str(),
                    is(equal_to(":test!guest@127.0.0.1 PRIVMSG test test\r\n")));
            }
        }
    }

}
