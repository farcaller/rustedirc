/// Dummy reply number. Not used.
pub const RPL_NONE: u32 = 300;

/// Reply format used by USERHOST to list replies to
/// the query list.  The reply string is composed as
/// follows:
///
/// <reply> ::= <nick>['*'] '=' <'+'|'-'><hostname>
///
/// The '*' indicates whether the client has registered
/// as an Operator.  The '-' or '+' characters represent
/// whether the client has set an AWAY message or not
/// respectively.
pub const RPL_USERHOST: u32 = 302;  // ":[<reply>{<space><reply>}]"

/// Reply format used by ISON to list replies to the
/// query list.
pub const RPL_ISON: u32 = 303;  // ":[<nick> {<space><nick>}]"

/// These replies are used with the AWAY command (if
/// allowed).  RPL_AWAY is sent to any client sending a
/// PRIVMSG to a client which is away.  RPL_AWAY is only
/// sent by the server to which the client is connected.
/// Replies RPL_UNAWAY and RPL_NOWAWAY are sent when the
/// client removes and sets an AWAY message.
pub const RPL_AWAY: u32 = 301;  // "<nick> :<away message>"
pub const RPL_UNAWAY: u32 = 305;  // ":You are no longer marked as being away"
pub const RPL_NOWAWAY: u32 = 306;  // ":You have been marked as being away"

/// Replies 311 - 313, 317 - 319 are all replies
/// generated in response to a WHOIS message.  Given that
/// there are enough parameters present, the answering
/// server must either formulate a reply out of the above
/// numerics (if the query nick is found) or return an
/// error reply.  The '*' in RPL_WHOISUSER is there as
/// the literal character and not as a wild card.  For
/// each reply set, only RPL_WHOISCHANNELS may appear
/// more than once (for long lists of channel names).
/// The '@' and '+' characters next to the channel name
/// indicate whether a client is a channel operator or
/// has been granted permission to speak on a moderated
/// channel.  The RPL_ENDOFWHOIS reply is used to mark
/// the end of processing a WHOIS message.
pub const RPL_WHOISUSER: u32 = 311;  // "<nick> <user> <host> * :<real name>"
pub const RPL_WHOISSERVER: u32 = 312;  // "<nick> <server> :<server info>"
pub const RPL_WHOISOPERATOR: u32 = 313;  // "<nick> :is an IRC operator"
pub const RPL_WHOISIDLE: u32 = 317;  // "<nick> <integer> :seconds idle"
pub const RPL_ENDOFWHOIS: u32 = 318;  // "<nick> :End of /WHOIS list"
pub const RPL_WHOISCHANNELS: u32 = 319;  // "<nick> :{[@|+]<channel><space>}"

/// When replying to a WHOWAS message, a server must use
/// the replies RPL_WHOWASUSER, RPL_WHOISSERVER or
/// ERR_WASNOSUCHNICK for each nickname in the presented
/// list.  At the end of all reply batches, there must
/// be RPL_ENDOFWHOWAS (even if there was only one reply
/// and it was an error).
pub const RPL_WHOWASUSER: u32 = 314;  // "<nick> <user> <host> * :<real name>"
pub const RPL_ENDOFWHOWAS: u32 = 369;  // "<nick> :End of WHOWAS"

/// Replies RPL_LISTSTART, RPL_LIST, RPL_LISTEND mark
/// the start, actual replies with data and end of the
/// server's response to a LIST command.  If there are
/// no channels available to return, only the start
/// and end reply must be sent.
pub const RPL_LISTSTART: u32 = 321;  // "Channel :Users  Name"
pub const RPL_LIST: u32 = 322;  // "<channel> <# visible> :<topic>"
pub const RPL_LISTEND: u32 = 323;  // ":End of /LIST"

pub const RPL_CHANNELMODEIS: u32 = 324;  // "<channel> <mode> <mode params>"

/// When sending a TOPIC message to determine the
/// channel topic, one of two replies is sent.  If
/// the topic is set, RPL_TOPIC is sent back else
/// RPL_NOTOPIC.
pub const RPL_NOTOPIC: u32 = 331;  // "<channel> :No topic is set"
pub const RPL_TOPIC: u32 = 332;  // "<channel> :<topic>"

/// Returned by the server to indicate that the
/// attempted INVITE message was successful and is
/// being passed onto the end client.
pub const RPL_INVITING: u32 = 341;  // "<channel> <nick>"

/// Returned by a server answering a SUMMON message to
/// indicate that it is summoning that user.
pub const RPL_SUMMONING: u32 = 342;  // "<user> :Summoning user to IRC"

/// Reply by the server showing its version details.
/// The <version> is the version of the software being
/// used (including any patchlevel revisions) and the
/// <debuglevel> is used to indicate if the server is
/// running in "debug mode".
/// The "comments" field may contain any comments about
/// the version or further version details.
pub const RPL_VERSION: u32 = 351;  // "<version>.<debuglevel> <server> :<comments>"

/// The RPL_WHOREPLY and RPL_ENDOFWHO pair are used
/// to answer a WHO message.  The RPL_WHOREPLY is only
/// sent if there is an appropriate match to the WHO
/// query.  If there is a list of parameters supplied
/// with a WHO message, a RPL_ENDOFWHO must be sent
/// after processing each list item with <name> being
/// the item.
pub const RPL_WHOREPLY: u32 = 352;  // "<channel> <user> <host> <server> <nick> <H|G>[*][@|+] :<hopcount> <real name>"
pub const RPL_ENDOFWHO: u32 = 315;  // "<name> :End of /WHO list"

/// To reply to a NAMES message, a reply pair consisting
/// of RPL_NAMREPLY and RPL_ENDOFNAMES is sent by the
/// server back to the client.  If there is no channel
/// found as in the query, then only RPL_ENDOFNAMES is
/// returned.  The exception to this is when a NAMES
/// message is sent with no parameters and all visible
/// channels and contents are sent back in a series of
/// RPL_NAMEREPLY messages with a RPL_ENDOFNAMES to mark
/// the end.
pub const RPL_NAMREPLY: u32 = 353;  // "<channel> :[[@|+]<nick> [[@|+]<nick> [...]]]"
pub const RPL_ENDOFNAMES: u32 = 366;  // "<channel> :End of /NAMES list"

/// In replying to the LINKS message, a server must send
/// replies back using the RPL_LINKS numeric and mark the
/// end of the list using an RPL_ENDOFLINKS reply.
pub const RPL_LINKS: u32 = 364;  // "<mask> <server> :<hopcount> <server info>"
pub const RPL_ENDOFLINKS: u32 = 365;  // "<mask> :End of /LINKS list"

/// When listing the active 'bans' for a given channel,
/// a server is required to send the list back using the
/// RPL_BANLIST and RPL_ENDOFBANLIST messages.  A separate
/// RPL_BANLIST is sent for each active banid.  After the
/// banids have been listed (or if none present) a
/// RPL_ENDOFBANLIST must be sent.
pub const RPL_BANLIST: u32 = 367;  // "<channel> <banid>"
pub const RPL_ENDOFBANLIST: u32 = 368;  // "<channel> :End of channel ban list"

/// A server responding to an INFO message is required to
/// send all its 'info' in a series of RPL_INFO messages
/// with a RPL_ENDOFINFO reply to indicate the end of the
/// replies.
pub const RPL_INFO: u32 = 371;  // ":<string>"
pub const RPL_ENDOFINFO: u32 = 374;  // ":End of /INFO list"

/// When responding to the MOTD message and the MOTD file
/// is found, the file is displayed line by line, with
/// each line no longer than 80 characters, using
/// RPL_MOTD format replies.  These should be surrounded
/// by a RPL_MOTDSTART (before the RPL_MOTDs) and an
/// RPL_ENDOFMOTD (after).
pub const RPL_MOTDSTART: u32 = 375;  // ":- <server> Message of the day - "
pub const RPL_MOTD: u32 = 372;  // ":- <text>"
pub const RPL_ENDOFMOTD: u32 = 376;  // ":End of /MOTD command"

/// RPL_YOUREOPER is sent back to a client which has
/// just successfully issued an OPER message and gained
/// operator status.
pub const RPL_YOUREOPER: u32 = 381;  // ":You are now an IRC operator"

/// If the REHASH option is used and an operator sends
/// a REHASH message, an RPL_REHASHING is sent back to
/// the operator.
pub const RPL_REHASHING: u32 = 382;  // "<config file> :Rehashing"

/// When replying to the TIME message, a server must send
/// the reply using the RPL_TIME format above.  The string
/// showing the time need only contain the correct day and
/// time there.  There is no further requirement for the
/// time string.
pub const RPL_TIME: u32 = 391;  // "<server> :<string showing server's local time>"

/// If the USERS message is handled by a server, the
/// replies RPL_USERSTART, RPL_USERS, RPL_ENDOFUSERS and
/// RPL_NOUSERS are used.  RPL_USERSSTART must be sent
/// first, following by either a sequence of RPL_USERS
/// or a single RPL_NOUSER.  Following this is
/// RPL_ENDOFUSERS.
pub const RPL_USERSSTART: u32 = 392;  // ":UserID   Terminal  Host"
pub const RPL_USERS: u32 = 393;  // ":%-8s %-9s %-8s"
pub const RPL_ENDOFUSERS: u32 = 394;  // ":End of users"
pub const RPL_NOUSERS: u32 = 395;  // ":Nobody logged in"

/// The RPL_TRACE* are all returned by the server in
/// response to the TRACE message.  How many are
/// returned is dependent on the the TRACE message and
/// whether it was sent by an operator or not.  There
/// is no predefined order for which occurs first.
/// Replies RPL_TRACEUNKNOWN, RPL_TRACECONNECTING and
/// RPL_TRACEHANDSHAKE are all used for connections
/// which have not been fully established and are either
/// unknown, still attempting to connect or in the
/// process of completing the 'server handshake'.
/// RPL_TRACELINK is sent by any server which handles
/// a TRACE message and has to pass it on to another
/// server.  The list of RPL_TRACELINKs sent in
/// response to a TRACE command traversing the IRC
/// network should reflect the actual connectivity of
/// the servers themselves along that path.
/// RPL_TRACENEWTYPE is to be used for any connection
/// which does not fit in the other categories but is
/// being displayed anyway.
pub const RPL_TRACELINK: u32 = 200;  // "Link <version & debug level> <destination> <next server>"
pub const RPL_TRACECONNECTING: u32 = 201;  // "Try. <class> <server>"
pub const RPL_TRACEHANDSHAKE: u32 = 202;  // "H.S. <class> <server>"
pub const RPL_TRACEUNKNOWN: u32 = 203;  // "???? <class> [<client IP address in dot form>]"
pub const RPL_TRACEOPERATOR: u32 = 204;  // "Oper <class> <nick>"
pub const RPL_TRACEUSER: u32 = 205;  // "User <class> <nick>"
pub const RPL_TRACESERVER: u32 = 206;  // "Serv <class> <int>S <int>C <server> <nick!user|*!*>@<host|server>"
pub const RPL_TRACENEWTYPE: u32 = 208;  // "<newtype> 0 <client name>"
pub const RPL_TRACELOG: u32 = 261;  // "File <logfile> <debug level>"

pub const RPL_STATSLINKINFO: u32 = 211;  // "<linkname> <sendq> <sent messages> <sent bytes> <received messages> <received bytes> <time open>"
pub const RPL_STATSCOMMANDS: u32 = 212;  // "<command> <count>"
pub const RPL_STATSCLINE: u32 = 213;  // "C <host> * <name> <port> <class>"
pub const RPL_STATSNLINE: u32 = 214;  // "N <host> * <name> <port> <class>"
pub const RPL_STATSILINE: u32 = 215;  // "I <host> * <host> <port> <class>"
pub const RPL_STATSKLINE: u32 = 216;  // "K <host> * <username> <port> <class>"
pub const RPL_STATSYLINE: u32 = 218;  // "Y <class> <ping frequency> <connect frequency> <max sendq>"
pub const RPL_ENDOFSTATS: u32 = 219;  // "<stats letter> :End of /STATS report"
pub const RPL_STATSLLINE: u32 = 241;  // "L <hostmask> * <servername> <maxdepth>"
pub const RPL_STATSUPTIME: u32 = 242;  // ":Server Up %d days %d:%02d:%02d"
pub const RPL_STATSOLINE: u32 = 243;  // "O <hostmask> * <name>"
pub const RPL_STATSHLINE: u32 = 244;  // "H <hostmask> * <servername>"

/// To answer a query about a client's own mode,
/// RPL_UMODEIS is sent back.
pub const RPL_UMODEIS: u32 = 221;  // "<user mode string>"

/// In processing an LUSERS message, the server
/// sends a set of replies from RPL_LUSERCLIENT,
/// RPL_LUSEROP, RPL_USERUNKNOWN,
/// RPL_LUSERCHANNELS and RPL_LUSERME.  When
/// replying, a server must send back
/// RPL_LUSERCLIENT and RPL_LUSERME.  The other
/// replies are only sent back if a non-zero count
/// is found for them.
pub const RPL_LUSERCLIENT: u32 = 251;  // ":There are <integer> users and <integer> invisible on <integer> servers"
pub const RPL_LUSEROP: u32 = 252;  // "<integer> :operator(s) online"
pub const RPL_LUSERUNKNOWN: u32 = 253;  // "<integer> :unknown connection(s)"
pub const RPL_LUSERCHANNELS: u32 = 254;  // "<integer> :channels formed"
pub const RPL_LUSERME: u32 = 255;  // ":I have <integer> clients and <integer> servers"

/// When replying to an ADMIN message, a server
/// is expected to use replies RLP_ADMINME
/// through to RPL_ADMINEMAIL and provide a text
/// message with each.  For RPL_ADMINLOC1 a
/// description of what city, state and country
/// the server is in is expected, followed by
/// details of the university and department
/// (RPL_ADMINLOC2) and finally the administrative
/// contact for the server (an email address here
/// is required) in RPL_ADMINEMAIL.
pub const RPL_ADMINME: u32 = 256;  // "<server> :Administrative info"
pub const RPL_ADMINLOC1: u32 = 257;  // ":<admin info>"
pub const RPL_ADMINLOC2: u32 = 258;  // ":<admin info>"
pub const RPL_ADMINEMAIL: u32 = 259;  // ":<admin info>"
