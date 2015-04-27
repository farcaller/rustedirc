mod nick;
mod user;
mod privmsg;

use message::Message;
use server::{Server, Client};

impl Server {
    pub fn resolve_command(&self, message: Message, client: &Client) {
        match message.command.to_uppercase().as_str() {
            "NICK" => self.cmd_nick(message, client),
            "USER" => self.cmd_user(message, client),
            "PRIVMSG" => self.cmd_privmsg(message, client),
            _ => ()
        }
    }
}
