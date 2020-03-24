extern crate irc;
extern crate tokio_postgres;

use irc::client::prelude::{Client, ClientExt, Command, IrcClient};
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("[DEBUG] Connecting to IRC server...");
    let client = IrcClient::new("config.toml").unwrap();

    // Connect to the database.
    let (pg_client, connection) = tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
        .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = pg_client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    client.identify().unwrap();
    client.for_each_incoming(|im| {

        if let Command::NOTICE(channel, message) = &im.command {
            println!("[{:?}][{}]: {}", &im.response_target(), &channel, &message)
        }

        if let Command::PRIVMSG(channel, message) = &im.command {
            let src_nick = &im.source_nickname().unwrap();

            let help_msg = vec![
                format!("Hey there {}, here's what you can do.", &im.source_nickname().unwrap()),
                "Say '!create' followed by the title of your listing to create a new one.".to_string(),
                "Example: '!create I am offering virtual guitar lessons/sessions'.".to_string(),
                "Say '!delete' followed by the name or ID of an existing listing.".to_string(),
                "Example: '!delete I am offering virtual guitar lessons/sessions'".to_string()
            ];
            let help_msg = help_msg.join(" ");

            // Print all messages
            println!(
                "[{}][{}]: {}",
                &im.response_target().unwrap(),
                src_nick,
                message
            );

            if message.starts_with("!create") {
                let user_msg = message.split(" ");
                let mut user_msg: Vec<&str> = user_msg.collect();
                user_msg.remove(0);
                let user_msg = user_msg.join(" ");
                println!("it looks like you said: '{:?}' - Trying to save to the DB", user_msg);
                // pg_client.query(
                //     "INSERT INTO posts (nick, post_title) VALUES ('$1', '$2')",
                //     &[src_nick, &user_msg]
                // );
                // let value: &str = rows2[0].get(0);
                // println!("{}", value);
            } else if message.starts_with("!delete") {
                let _ = client.send_privmsg(&channel, "This feature is still being worked on.");
            } else if message.starts_with("!help") {
                let _ = client.send_privmsg(&channel, &help_msg);
            }
        }
    }).unwrap();

    Ok(())
}
