// pub use server::*;
// pub use hamcrest::{assert_that, is, not, none, equal_to};

// describe! server {
//     before_each {
//         let mut server = Server::new("test.local".to_string(), "TestLocal".to_string(),"42X".to_string());
//     }

//     describe! accepted_connection {
//         before_each {
//             let sock = Box::new(Vec::new());
//             let token = 8;

//             server.accept_connection(sock, token);
//         }

//         it "is resolved by token" {
//             assert_that(server.client_by_token(8), is(not(none())));
//         }

//         it "sets user nick when NICK is passed" {
//             server.process_line(token, "NICK test");

//             let user = server.client_by_token(8).unwrap();

//             assert_that(user.nickname().unwrap().as_str(), is(equal_to("test")));
//         }

//         it "updates user nick when NICK is passed" {
//             server.process_line(token, "NICK test");

//             let user = server.client_by_token(8).unwrap();

//             assert_that(user.nickname().unwrap().as_str(), is(equal_to("test")));
//         }
//     }
// }
