extern crate irc;
extern crate postgres;

use irc::client::prelude::{ClientExt, Command, Config, IrcReactor};
use postgres::{NoTls, Client};


fn main() {
    // Connect to IRC server
    println!("[DEBUG] Connecting to IRC server...");
    let irc_config = Config::load("config.toml").unwrap();
    let mut reactor = IrcReactor::new().unwrap();
    let irc_client = reactor.prepare_client_and_connect(&irc_config).unwrap();
    irc_client.identify().unwrap();

    reactor.register_client_with_handler(irc_client, |c, m| {
        // Connect to database
        let mut pg_client = Client::connect("host=localhost user=postgres password=postgres dbname=communitybot", NoTls).unwrap();

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
                let query_res = pg_client.query_one(
                    "INSERT INTO posts (nick, post_title) VALUES ($1, $2) RETURNING id",
                    &[&src_nick, &user_msg]
                );
                match query_res {
                    Ok(row) => {
                        let post_id: i32 = row.get("id");
                        c.send_privmsg(&channel, format!("Created new post: {}!", post_id));
                    },
                    Err(err) => {
                        c.send_privmsg(&channel, format!("There was an error storing to DB! {}", err));
                    }
                };
            } else if message.starts_with("!delete") {
                let _ = c.send_privmsg(&channel, "This feature is still being worked on.");
            } else if message.starts_with("!help") {
                let _ = c.send_privmsg(&channel, &help_msg);
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
}
