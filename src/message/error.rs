// use server::{Server, Client};
use std::{fmt, error};

pub use super::numerics::rfc1459::error_replies::*;
pub use super::numerics::rfc1459::command_replies::*;

#[derive(Debug)]
pub struct IRCError {
    raw_args: Option<String>,
    error_code: u32,
}

impl IRCError {
    pub fn new(raw_args: &str, error_code: u32) -> IRCError {
        IRCError {
            raw_args: Some(raw_args.to_string()),
            error_code: error_code,
        }
    }

    pub fn ignore() -> IRCError {
        IRCError {
            raw_args: None,
            error_code: 0,
        }
    }

    pub fn need_more_params(command: &str) -> IRCError {
        IRCError {
            raw_args: Some(format!("{} :Not enough parameters", command)),
            error_code: ERR_NEEDMOREPARAMS,
        }
    }

    // pub fn to_message(&self, server: &Server, client: &Client) -> String {
    //     format!(":{} {} {} {}", server.name(), self.error_code,
    //         ctx.user().identifier(), self.raw_args.as_ref().unwrap())
    // }
}

impl fmt::Display for IRCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.error_code, self.raw_args.as_ref().unwrap())
    }
}

impl error::Error for IRCError {
    fn description<'a>(&self) -> &str {
        "processing the command"
        // format!("{} {}", self.error_code, self.raw_args.as_ref().unwrap()).as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
