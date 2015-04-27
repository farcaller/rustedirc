use std::fmt;

pub mod error;
mod numerics;

pub struct Message<'a> {
    pub prefix: Option<&'a str>,
    pub command: &'a str,
    pub arguments: Vec<&'a str>,
}

// pub type MessageProcessor = fn(ctx: &mut Context, message: &Message) -> Result<(), error::IRCError>;

// pub fn resolve(line: &str) -> Result<(Message, MessageProcessor), error::IRCError> {
//     let raw_message = if let Some(msg) = Message::new(line) {
//         msg
//     } else {
//         return Err(error::IRCError::ignore())  // TODO: unparseable line
//     };

//     match raw_message.command.to_uppercase().as_str() {
//         // "PASS" => Ok((raw_message, password::process_pass)),
//         "NICK" => Ok((raw_message, nick::process_nick)),
//         // "USER" => Ok((raw_message, user::process_user)),
//         _ => Err(error::IRCError::ignore()) // TODO: (format!("unknown command '{}'", other).as_str()))
//     }
// }

// pub fn process(server: &Server, user: &UserClient, line: &str) -> Result<(), error::IRCError> {
//     let (raw_message, handler) = try!(resolve(line));
//     handler(server, user, &raw_message)
// }

impl<'a> Message<'a> {
    pub fn build(prefix: Option<&'a str>, command: &'static str, arguments: Vec<&'a str>) -> Message<'a> {
        Message {
            prefix: prefix,
            command: command,
            arguments: arguments
        }
    }

    pub fn new(line: &'a str) -> Option<Message> {
        let mut remains = line;

        let command: &'a str;
        let mut arguments = vec![];

        let first_char = if let Some(first) = remains.chars().next() {
            first
        } else {
            return None
        };

        let prefix = if first_char == ':' {
            if let Some(command_start) = remains.find(' ') {
                let prefix = &remains[1..command_start];
                remains = &remains[command_start+1..];
                Some(prefix)
            } else {
                return None;
            }
        } else {
            None
        };

        if let Some(args_start) = remains.find(' ') {
            command = &remains[..args_start];

            remains = &remains[args_start+1..];
            loop {
                let first_char = if let Some(first) = remains.chars().next() {
                    first
                } else {
                    break
                };

                if first_char == ':' {
                    arguments.push(&remains[1..]);
                    break;
                } else {
                    if let Some(arg_end) = remains.find(' ') {
                        arguments.push(&remains[..arg_end]);
                        remains = &remains[arg_end+1..];
                    } else {
                        arguments.push(remains);
                        break;
                    }
                }
            }
        } else {
            command = remains;
        }
        Some(Message {
            prefix: prefix,
            command: command,
            arguments: arguments,
        })
    }
}

impl<'a> fmt::Debug for Message<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(prefix) = self.prefix {
            try!(fmt.write_fmt(format_args!(":{} ", prefix)));
        }
        try!(fmt.write_str(self.command));

        for arg in self.arguments.iter() {
            if arg.find(' ') == None {
                try!(fmt.write_fmt(format_args!(" {}", arg)));
            } else {
                try!(fmt.write_fmt(format_args!(" :{}", arg)));
                return Ok(())
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    pub use super::Message;
    pub use hamcrest::{assert_that, is, not, none, equal_to};

    describe! parse {
        it "parses a simple line successfully" {
            let m = Message::new("REHASH");
            assert_that(m, is(not(none::<Message>())));
        }

        it "resolves command for a simple line" {
            let m = Message::new("REHASH");
            assert_that(m.unwrap().command, is(equal_to("REHASH")));
        }

        it "parses a line with arguments successfully" {
            let m = Message::new("NICK hello");
            assert_that(m, is(not(none::<Message>())));
        }

        it "resolves command for a line with arguments" {
            let m = Message::new("NICK hello");
            assert_that(m.unwrap().command, is(equal_to("NICK")));
        }

        it "resolves arguments for a line with arguments" {
            let m = Message::new("NICK hello");
            assert_that(m.unwrap().arguments, is(equal_to(vec!["hello"])));
        }

        it "resolves several arguments for a line with arguments" {
            let m = Message::new("INVITE test #somewhere");
            assert_that(m.unwrap().arguments, is(equal_to(vec!["test", "#somewhere"])));
        }

        it "resolves trailing argument for a line with arguments" {
            let m = Message::new("USER guest 0 * :Ronnie Reagan");
            assert_that(m.unwrap().arguments, is(equal_to(vec!["guest", "0", "*", "Ronnie Reagan"])));
        }

        it "parses a servername prefix" {
            let m = Message::new(":my.local.server PONG");
            assert_that(m.unwrap().prefix, is(equal_to(Some("my.local.server"))));
        }

        it "returns None if the command is malformed" {
            let m = Message::new(":no_command");
            assert_that(m, is(none::<Message>()));
        }
    }
}
