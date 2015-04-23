use message::Message;
use message::error::*;
use context::Context;

/// 4.1.1 Password message
///
///    Command: PASS
///    Parameters: <password>
///
///    The PASS command is used to set a 'connection password'.  The
///    password can and must be set before any attempt to register the
///    connection is made.  Currently this requires that clients send a PASS
///    command before sending the NICK/USER combination and servers *must*
///    send a PASS command before any SERVER command.  The password supplied
///    must match the one contained in the C/N lines (for servers) or I
///    lines (for clients).  It is possible to send multiple PASS commands
///    before registering but only the last one sent is used for
///    verification and it may not be changed once registered.  Numeric
///    Replies:
///
///    ERR_NEEDMOREPARAMS              ERR_ALREADYREGISTRED
///
///    Example:
///
///    PASS secretpasswordhere
pub fn process_pass(ctx: &mut Context, message: &Message) -> Result<(), IRCError> {
    if message.arguments.len() != 1 {
        Err(IRCError::need_more_params("PASS"))
    } else if ctx.user().registered() {
        Err(IRCError::new(":You may not reregister", ERR_ALREADYREGISTRED))
    } else {
        ctx.user().set_password(message.arguments[0]);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    pub use super::process_pass;
    pub use message::{resolve, process, MessageProcessor};
    pub use context::*;
    pub use hamcrest::{assert_that, is, not, none, equal_to};

    pub struct MockContext {
        password: String,
        registered: bool,
    }

    impl MockContext {
        pub fn new() -> MockContext {
            MockContext {
                password: String::new(),
                registered: false,
            }
        }
    }

    impl Context for MockContext {
        fn get_user(&self, _: &str) -> Option<&UserContext> { panic!() }
        fn user(&mut self) -> &mut UserContext { self }
        fn server_name(&self) -> String { "server.local".to_string() }
    }

    impl UserContext for MockContext {
        fn identifier(&self) -> String {
            "*".to_string()
        }
        fn registered(&self) -> bool {
            self.registered
        }
        fn set_password(&mut self, password: &str) {
            self.password = password.to_string();
        }
        fn set_nickname(&mut self, _: &str) { panic!() }
    }

    describe! password {
        before_each {
            let mut ctx = &mut MockContext::new();
            ctx.user();  // silence warning
        }

        it "resolves the correct handler" {
            let (_, handler) = resolve("PASS").ok().unwrap();
            assert_that(handler as *const usize, is(equal_to(process_pass as *const usize)));
        }

        it "updates the password in Context" {
            assert_that(process(ctx as &mut Context, "PASS test").is_ok(), is(equal_to(true)));
            assert_that(ctx.password.as_str(), is(equal_to("test")));
        }

        it "fails on incorrect argument count" {
            let err = process(ctx as &mut Context, "PASS");
            assert_that(err.is_err(), is(equal_to(true)));
            assert_that(err.err().unwrap().to_message(ctx as &mut Context),
                is(equal_to(":server.local 461 * PASS :Not enough parameters".to_string())));
        }

        it "fails if already registered" {
            ctx.registered = true;
            let err = process(ctx as &mut Context, "PASS test");
            assert_that(err.is_err(), is(equal_to(true)));
            assert_that(err.err().unwrap().to_message(ctx as &mut Context),
                is(equal_to(":server.local 462 * :You may not reregister".to_string())));
        }
    }
}
