/// Used to indicate the nickname parameter supplied to a command is currently unused.
pub const ERR_NOSUCHNICK: u32 = 401;  // "<nickname> :No such nick/channel"

/// Used to indicate the server name given currently doesn't exist.
pub const ERR_NOSUCHSERVER: u32 = 402;  // "<server name> :No such server"

/// Used to indicate the given channel name is invalid.
pub const ERR_NOSUCHCHANNEL: u32 = 403;  // "<channel name> :No such channel"

/// Sent to a user who is either (a) not on a channel
/// which is mode +n or (b) not a chanop (or mode +v) on
/// a channel which has mode +m set and is trying to send
/// a PRIVMSG message to that channel.
pub const ERR_CANNOTSENDTOCHAN: u32 = 404;  // "<channel name> :Cannot send to channel"

/// Sent to a user when they have joined the maximum
/// number of allowed channels and they try to join
/// another channel.
pub const ERR_TOOMANYCHANNELS: u32 = 405;  // "<channel name> :You have joined too many channels"

/// Returned by WHOWAS to indicate there is no history
/// information for that nickname.
pub const ERR_WASNOSUCHNICK: u32 = 406;  // "<nickname> :There was no such nickname"

/// Returned to a client which is attempting to send a
/// PRIVMSG/NOTICE using the user@host destination format
/// and for a user@host which has several occurrences.
pub const ERR_TOOMANYTARGETS: u32 = 407;  // "<target> :Duplicate recipients. No message delivered"

/// PING or PONG message missing the originator parameter
/// which is required since these commands must work
/// without valid prefixes.
pub const ERR_NOORIGIN: u32 = 409;  // ":No origin specified"

/// 412 - 414 are returned by PRIVMSG to indicate that
/// the message wasn't delivered for some reason.
/// ERR_NOTOPLEVEL and ERR_WILDTOPLEVEL are errors that
/// are returned when an invalid use of
/// "PRIVMSG $<server>" or "PRIVMSG #<host>" is attempted.
pub const ERR_NORECIPIENT: u32 = 411;  // ":No recipient given (<command>)"

pub const ERR_NOTEXTTOSEND: u32 = 412;  // ":No text to send"

pub const ERR_NOTOPLEVEL: u32 = 413;  // "<mask> :No toplevel domain specified"

pub const ERR_WILDTOPLEVEL: u32 = 414;  // "<mask> :Wildcard in toplevel domain"

/// Returned to a registered client to indicate that the
/// command sent is unknown by the server.
pub const ERR_UNKNOWNCOMMAND: u32 = 421;  // "<command> :Unknown command"

/// Server's MOTD file could not be opened by the server.
pub const ERR_NOMOTD: u32 = 422;  // ":MOTD File is missing"

/// Returned by a server in response to an ADMIN message
/// when there is an error in finding the appropriate
/// information.
pub const ERR_NOADMININFO: u32 = 423;  // "<server> :No administrative info available"

/// Generic error message used to report a failed file
/// operation during the processing of a message.
pub const ERR_FILEERROR: u32 = 424;  // ":File error doing <file op> on <file>"

/// Returned when a nickname parameter expected for a
/// command and isn't found.
pub const ERR_NONICKNAMEGIVEN: u32 = 431;  // ":No nickname given"

/// Returned after receiving a NICK message which contains
/// characters which do not fall in the defined set.  See
/// section x.x.x for details on valid nicknames.
pub const ERR_ERRONEUSNICKNAME: u32 = 432;  // "<nick> :Erroneus nickname"

/// Returned when a NICK message is processed that results
/// in an attempt to change to a currently existing
/// nickname.
pub const ERR_NICKNAMEINUSE: u32 = 433;  // "<nick> :Nickname is already in use"

/// Returned by a server to a client when it detects a
/// nickname collision (registered of a NICK that
/// already exists by another server).
pub const ERR_NICKCOLLISION: u32 = 436;  // "<nick> :Nickname collision KILL"

/// Returned by the server to indicate that the target
/// user of the command is not on the given channel.
pub const ERR_USERNOTINCHANNEL: u32 = 441;  // "<nick> <channel> :They aren't on that channel"

/// Returned by the server whenever a client tries to
/// perform a channel effecting command for which the
/// client isn't a member.
pub const ERR_NOTONCHANNEL: u32 = 442;  // "<channel> :You're not on that channel"

/// Returned when a client tries to invite a user to a
/// channel they are already on.
pub const ERR_USERONCHANNEL: u32 = 443;  // "<user> <channel> :is already on channel"

//// Returned by the summon after a SUMMON command for a
//// user was unable to be performed since they were not
//// logged in.
pub const ERR_NOLOGIN: u32 = 444;  // "<user> :User not logged in"

/// Returned as a response to the SUMMON command.  Must be
/// returned by any server which does not implement it.
pub const ERR_SUMMONDISABLED: u32 = 445;  // ":SUMMON has been disabled"

/// Returned as a response to the USERS command.  Must be
/// returned by any server which does not implement it.
pub const ERR_USERSDISABLED: u32 = 446;  // ":USERS has been disabled"

/// Returned by the server to indicate that the client
/// must be registered before the server will allow it
/// to be parsed in detail.
pub const ERR_NOTREGISTERED: u32 = 451;  // ":You have not registered"

/// Returned by the server by numerous commands to
/// indicate to the client that it didn't supply enough
/// parameters.
pub const ERR_NEEDMOREPARAMS: u32 = 461;  // "<command> :Not enough parameters"

/// Returned by the server to any link which tries to
/// change part of the registered details (such as
/// password or user details from second USER message).
pub const ERR_ALREADYREGISTRED: u32 = 462;  // ":You may not reregister"

/// Returned to a client which attempts to register with
/// a server which does not been setup to allow
/// connections from the host the attempted connection
/// is tried.
pub const ERR_NOPERMFORHOST: u32 = 463;  // ":Your host isn't among the privileged"

/// Returned to indicate a failed attempt at registering
/// a connection for which a password was required and
/// was either not given or incorrect.
pub const ERR_PASSWDMISMATCH: u32 = 464;  // ":Password incorrect"

/// Returned after an attempt to connect and register
/// yourself with a server which has been setup to
/// explicitly deny connections to you.
pub const ERR_YOUREBANNEDCREEP: u32 = 465;  // ":You are banned from this server"

/// Any command requiring operator privileges to operate
/// must return this error to indicate the attempt was
/// unsuccessful.
pub const ERR_KEYSET: u32 = 467;  // "<channel> :Channel key already set"

pub const ERR_CHANNELISFULL: u32 = 471;  // "<channel> :Cannot join channel (+l)"

pub const ERR_UNKNOWNMODE: u32 = 472;  // "<char> :is unknown mode char to me"

pub const ERR_INVITEONLYCHAN: u32 = 473;  // "<channel> :Cannot join channel (+i)"

pub const ERR_BANNEDFROMCHAN: u32 = 474;  // "<channel> :Cannot join channel (+b)"

pub const ERR_BADCHANNELKEY: u32 = 475;  // "<channel> :Cannot join channel (+k)"

pub const ERR_NOPRIVILEGES: u32 = 481;  // ":Permission Denied- You're not an IRC operator"

/// Any command requiring 'chanop' privileges (such as
/// MODE messages) must return this error if the client
/// making the attempt is not a chanop on the specified
/// channel.
pub const ERR_CHANOPRIVSNEEDED: u32 = 482;  // "<channel> :You're not channel operator"

/// Any attempts to use the KILL command on a server
/// are to be refused and this error returned directly
/// to the client.
pub const ERR_CANTKILLSERVER: u32 = 483;  // ":You cant kill a server!"

/// If a client sends an OPER message and the server has
/// not been configured to allow connections from the
/// client's host as an operator, this error must be
/// returned.
pub const ERR_NOOPERHOST: u32 = 491;  // ":No O-lines for your host"

/// Returned by the server to indicate that a MODE
/// message was sent with a nickname parameter and that
/// the a mode flag sent was not recognized.
pub const ERR_UMODEUNKNOWNFLAG: u32 = 501;  // ":Unknown MODE flag"

/// Error sent to any user trying to view or change the
/// user mode for a user other than themselves.
pub const ERR_USERSDONTMATCH: u32 = 502;  // ":Cant change mode for other users"
