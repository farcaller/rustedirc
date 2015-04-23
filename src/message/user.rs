use message::Message;
use message::error::IRCError;
use context::Context;

pub fn process_user(ctx: &mut Context, message: &Message) -> Result<(), IRCError> {
    if message.arguments.len() != 4 {
        Err(IRCError::need_more_params("USER"))
    } else {
        Ok(())
    }
}
