use irc::client::prelude::{IrcClient, ClientExt, Client, Command};

fn main() {
    println!("[DEBUG] Connecting to IRC server...");
    let client = IrcClient::new("config.toml").unwrap();
    client.identify().unwrap();
    client.for_each_incoming(|irc_msg| {
        
        // Debug
        // println!("{}", &irc_msg);

        if let Command::NOTICE(channel, message) = &irc_msg.command {
            println!("[{:?}][{}]: {}", &irc_msg.response_target(), &channel, &message)
        }

        if let Command::PRIVMSG(channel, message) = &irc_msg.command {
            // Print all messages
            println!("[{}][{}]: {}", &irc_msg.response_target().unwrap(), &irc_msg.source_nickname().unwrap(), message);

            if message.contains(&format!("hey {}", client.current_nickname())) {
                let _ = client.send_privmsg(&channel, format!("hey there {}", &irc_msg.source_nickname().unwrap()));
            }
        }
    }).unwrap();
}
