use std::collections::HashMap;

pub trait UserContext {
    fn identifier(&self) -> String;
    fn registered(&self) -> bool;
    fn set_password(&mut self, password: &str);
    fn set_nickname(&mut self, nickname: &str);
}

pub trait Context {
    fn get_user(&self, username: &str) -> Option<&UserContext>;
    fn user(&mut self) -> &mut UserContext;
    fn server_name(&self) -> String;
}

pub struct ClientState {
    nickname: String,
}

pub struct ServerState {
    server_name: String,
    clients: HashMap<String, ClientState>
}

impl ServerState {
    pub fn new<'a>(server_name: String) -> ServerState {
        ServerState {
            server_name: server_name,
            clients: HashMap::new(),
        }
    }
}

pub struct ClientContext<'a> {
    server_state: &'a mut ServerState,
}

// impl<'a> Context for ClientContext<'a> {
//     fn get_user(&self, username: &str) -> Option<&UserContext> {
//         panic!()
//     }

//     fn user(&mut self) -> &mut UserContext {
//         panic!()
//     }

//     fn server_name(&self) -> &str {
//         self.server_state.server_name.as_str()
//     }
// }

// #[cfg(test)]
// mod test {
//     pub use std::borrow::ToOwned;
//     pub use super::*;
//     pub use hamcrest::{assert_that, is, not, none, equal_to};

//     describe! client_context {
//         it "returns the server name" {
//             let mut server_state = ServerState::new("test.local".to_owned());
//             let ctx = ClientContext {server_state: &mut server_state};

//             assert_that(ctx.server_name(), is(equal_to("test.local".to_owned())));
//         }
//     }
// }
