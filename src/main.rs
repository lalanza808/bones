extern crate irc;

use irc::client::prelude::{Client, ClientExt, Command, IrcClient};

fn main() {
    println!("[DEBUG] Connecting to IRC server...");
    let client = IrcClient::new("config.toml").unwrap();
    client.identify().unwrap();
    client.for_each_incoming(|im| {

        if let Command::NOTICE(channel, message) = &im.command {
            println!("[{:?}][{}]: {}", &im.response_target(), &channel, &message)
        }

        if let Command::PRIVMSG(channel, message) = &im.command {
            let help_msg = vec![
                format!("Hey there {}, here's what you can do.", &im.source_nickname().unwrap()),
                "Say 'create:' to create a new listing, ie, 'create:This will be the title of your new post on the site'.".to_string(),
                "Say 'delete:' to delete an existing listing, ie, 'delete:Name or ID of your post to delete'.".to_string(),
            ];
            let help_msg = help_msg.join(" ");

            // Print all messages
            println!(
                "[{}][{}]: {}",
                &im.response_target().unwrap(),
                &im.source_nickname().unwrap(),
                message
            );

            if message.starts_with("create:") {
                let _ = client.send_privmsg(
                    &channel,
                    "This feature is still being worked on.",
                );
            } else if message.starts_with("delete:") {
                let _ = client.send_privmsg(
                    &channel,
                    "This feature is still being worked on.",
                );
            } else if message.contains("!help") {
                let _ = client.send_privmsg(
                    &channel,
                    &help_msg,
                );
            }
        }
    }).unwrap();
}
