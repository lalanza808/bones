extern crate irc;
extern crate tokio_postgres;

use irc::client::prelude::{ClientExt, Command, Config, IrcReactor};
use tokio_postgres::{NoTls, Error, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("[DEBUG] Connecting to IRC server...");
    let irc_config = Config::load("config.toml").unwrap();
    let (pg_client, connection) = tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
        .await?;
    let mut reactor = IrcReactor::new().unwrap();
    let irc_client = reactor.prepare_client_and_connect(&irc_config).unwrap();
    irc_client.identify().unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    reactor.register_client_with_handler(irc_client, |c, m| {
        if let Command::NOTICE(channel, message) = &m.command {
            println!("[{:?}][{}]: {}", &m.response_target(), &channel, &message);
        }

        if let Command::PRIVMSG(channel, message) = &m.command {
            let src_nick = &m.source_nickname().unwrap();
            let res_target = &m.response_target().unwrap();

            let help_msg = vec![
                format!("Hey there {}, here's what you can do.", &m.source_nickname().unwrap()),
                "Say '!create' followed by the title of your listing to create a new one.".to_string(),
                "Example: '!create I am offering virtual guitar lessons/sessions'.".to_string(),
                "Say '!delete' followed by the name or ID of an existing listing.".to_string(),
                "Example: '!delete I am offering virtual guitar lessons/sessions'".to_string()
            ];
            let help_msg = help_msg.join(" ");

            // Print user messages to channel
            println!("[{}][{}]: {}", res_target, src_nick, message);

            if message.starts_with("!create") {
                let user_msg = message.split(" ");
                let mut user_msg: Vec<&str> = user_msg.collect();
                user_msg.remove(0);
                let user_msg = user_msg.join(" ");
                println!("it looks like you said: {:?} - Trying to save to the DB", user_msg);
                // save to db
            } else if message.starts_with("!delete") {
                let _ = c.send_privmsg(&channel, "This feature is still being worked on.");
            } else if message.starts_with("!help") {
                let _ = c.send_privmsg(&channel, &help_msg);
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
    Ok(())
}

// async fn insert_post(client: &Client, nick: &str, user_msg: &str) -> String {
//     let rows2 = pg_client.query(
//         "INSERT INTO posts (nick, post_title) VALUES ('$1', '$2')",
//         &[&src_nick, &user_msg]
//     ).await?;
//     let value: &str = rows2[0].get(0);
//     println!("{}", value);
// }
