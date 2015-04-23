use std::str::pattern::Pattern;

use message::Message;
use message::error::*;
use context::Context;

/// 4.1.2 Nick message
///
///    Command: NICK
///    Parameters: <nickname> [ <hopcount> ]
///
///    NICK message is used to give user a nickname or change the previous
///    one.  The <hopcount> parameter is only used by servers to indicate
///    how far away a nick is from its home server.  A local connection has
///    a hopcount of 0.  If supplied by a client, it must be ignored.
///
///    If a NICK message arrives at a server which already knows about an
///    identical nickname for another client, a nickname collision occurs.
///    As a result of a nickname collision, all instances of the nickname
///    are removed from the server's database, and a KILL command is issued
///    to remove the nickname from all other server's database. If the NICK
///    message causing the collision was a nickname change, then the
///    original (old) nick must be removed as well.
///
///    If the server recieves an identical NICK from a client which is
///    directly connected, it may issue an ERR_NICKCOLLISION to the local
///    client, drop the NICK command, and not generate any kills.
///    Numeric Replies:
///
///    ERR_NONICKNAMEGIVEN             ERR_ERRONEUSNICKNAME
///    ERR_NICKNAMEINUSE               ERR_NICKCOLLISION
///
///    Example:
///
///    NICK Wiz                        ; Introducing new nick "Wiz".
///
///    :WiZ NICK Kilroy                ; WiZ changed his nickname to Kilroy.
pub fn process_nick(ctx: &mut Context, message: &Message) -> Result<(), IRCError> {
    if message.arguments.len() != 1 {
        Err(IRCError::new(":No nickname given", ERR_NONICKNAMEGIVEN))
    } else if !is_nickname_valid(message.arguments[0]) {
        Err(IRCError::new(format!("{} :Erroneus nickname", message.arguments[0]).as_str(),
            ERR_ERRONEUSNICKNAME))
    } else if ctx.get_user(message.arguments[0]).is_some() {
        Err(IRCError::new(format!("{} :Nickname is already in use", message.arguments[0]).as_str(),
            ERR_NICKNAMEINUSE))
    } else {
        ctx.user().set_nickname(message.arguments[0]);
        Ok(())
    }
}

///    <nick>       ::= <letter> { <letter> | <number> | <special> }
///    <letter>     ::= 'a' ... 'z' | 'A' ... 'Z'
///    <number>     ::= '0' ... '9'
///    <special>    ::= '-' | '[' | ']' | '\' | '`' | '^' | '{' | '}'
const VALID_CHARS: &'static str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM0123456789-[]\\`^{}";

pub fn is_nickname_valid(nickname: &str) -> bool {
    for c in nickname.chars() {
        if !c.is_contained_in(VALID_CHARS) {return false}
    }
    true
}

#[cfg(test)]
mod test {
    pub use super::process_nick;
    pub use message::{resolve, process, MessageProcessor};
    pub use context::*;
    pub use hamcrest::{assert_that, is, not, none, equal_to};

    pub struct MockContext {
        nickname: String,
        registered: bool,
        return_user: &'static str,
    }

    impl MockContext {
        pub fn new() -> MockContext {
            MockContext {
                nickname: String::new(),
                registered: false,
                return_user: "",
            }
        }
    }

    impl Context for MockContext {
        fn get_user(&self, username: &str) -> Option<&UserContext> {
            if self.return_user.len() > 0 {
                assert_that(username, is(equal_to(self.return_user)));
                Some(self)
            } else {None}
        }
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
        fn set_password(&mut self, _: &str) { panic!() }
        fn set_nickname(&mut self, nickname: &str) {
            self.nickname = nickname.to_string();
        }
    }

    describe! nick {
        before_each {
            let mut ctx = &mut MockContext::new();
            ctx.user();  // silence warning
        }

        it "resolves the correct handler" {
            let (_, handler) = resolve("NICK").ok().unwrap();
            assert_that(handler as *const usize, is(equal_to(process_nick as *const usize)));
        }

        it "updates the nickname in Context" {
            assert_that(process(ctx as &mut Context, "NICK test").is_ok(), is(equal_to(true)));
            assert_that(ctx.nickname.as_str(), is(equal_to("test")));
        }

        it "fails if no nickname is passed" {
            let err = process(ctx as &mut Context, "NICK");
            assert_that(err.is_err(), is(equal_to(true)));
            assert_that(err.err().unwrap().to_message(ctx as &mut Context),
                is(equal_to(":server.local 431 * :No nickname given".to_string())));
        }

        it "fails if nickname is not valid" {
            let err = process(ctx as &mut Context, "NICK $%");
            assert_that(err.is_err(), is(equal_to(true)));
            assert_that(err.err().unwrap().to_message(ctx as &mut Context),
                is(equal_to(":server.local 432 * $% :Erroneus nickname".to_string())));
        }

        it "fails if nickname is in use" {
            ctx.return_user = "someuser";
            let err = process(ctx as &mut Context, "NICK someuser");
            assert_that(err.is_err(), is(equal_to(true)));
            assert_that(err.err().unwrap().to_message(ctx as &mut Context),
                is(equal_to(":server.local 433 * someuser :Nickname is already in use".to_string())));
        }

        it "fails on nick collision" {
            // TODO: implement ERR_NICKCOLLISION
        }
    }
}
