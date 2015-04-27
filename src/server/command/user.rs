use server::{Server, Client};
use message::Message;

impl Server {
    pub fn cmd_user(&self, message: Message, client: &Client) {
        client.set_username(message.arguments[0].to_string());
        client.set_realname(message.arguments[3].to_string());

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

        it "sets user name and full name when USER is passed" {
            server.process_line(token, "USER guest 0 * :Ronnie Reagan");

            let user = server.client_by_token(8).unwrap();

            assert_that(user.username().unwrap().as_str(), is(equal_to("guest")));
            assert_that(user.realname().unwrap().as_str(), is(equal_to("Ronnie Reagan")));
        }
    }

}
