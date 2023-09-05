use bincode::Error;
use poem::session::Session;
use rspotify::Token;

/// Reads a serialized user token.
pub fn read_token(session: &Session) -> Result<Token, Error> {
    let serialized_token: Vec<u8> = session.get("access_token").unwrap_or_default();
    let token = bincode::deserialize(&serialized_token[..])?;

    return Ok(token);
}

pub fn write_token(token: Token, session: &Session) -> Result<(), Error> {
    let serialized_token = bincode::serialize(&token)?;
    session.set("access_token", serialized_token);

    return Ok(());
}