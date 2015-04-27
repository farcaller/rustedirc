use server::{Server, Client};
use message::Message;

impl Server {
    pub fn cmd_nick(&self, message: Message, client: &Client) {
        client.set_nickname(message.arguments[0].to_string());
        if client.registered() {
            self.register_client(client);
        }
    }
}

#[cfg(test)]
mod test {
    pub use server::*;
    pub use hamcrest::{assert_that, is, not, none, equal_to};

    describe! accepted_connection {
        before_each {
            let mut server = Server::new("test.local".to_string(), "TestLocal".to_string(),"42X".to_string());

            let sock = Box::new(Vec::new());
            let token = 8;

            server.accept_connection(sock, token, "127.0.0.1".to_string());
        }

        it "sets user nick when NICK is passed" {
            server.process_line(token, "NICK test");

            let user = server.client_by_token(8).unwrap();

            assert_that(user.nickname().unwrap().as_str(), is(equal_to("test")));
        }

        it "updates user nick when NICK is passed twice" {
            server.process_line(token, "NICK test");
            server.process_line(token, "NICK test2");

            let user = server.client_by_token(8).unwrap();

            assert_that(user.nickname().unwrap().as_str(), is(equal_to("test2")));
        }
    }

}
