// use std::env;
#[macro_use]
extern crate rss;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use rss::Channel;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // let urls = [
        // "https://note.mu/usop/rss",
        // "https://gizmodo.com/rss",
        // "https://www.cnet.com/rss/all/",
        // "https://techcrunch.com/feed/",
        // "https://news.ycombinator.com/rss",
        // "http://feeds.arstechnica.com/arstechnica/index/",
        // "http://feeds.mashable.com/Mashable",
        // "https://hub.packtpub.com/feed/",
        // "https://www.forbes.com/innovation/feed2/",
    // ];

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        let url = "https://gizmodo.com/rss";

        eprintln!("guild_id = {:?}", msg.guild_id);
        // let channel_name = msg.channel_id.name(&ctx.cache).await;
        // eprintln!("channel_name = {:?}", channel_name);
        let content = "Hello, world";
        if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
            println!("Error sending message: {:?}", why);
        }
        eprintln!();

        if msg.content.contains("話題") {
            println!("ヒット");
            // let links: Vec<FeedItem> = create_links(create_list(url));
            let links: Vec<FeedItem> = create_list(url);

            let mut length;
            if links.len() > 3 {
                length = 3;
            }else {
                length = links.len();
            }

            for index in 0..=length {
                let link = &links[index].link;
                msg.channel_id.say(&ctx.http, link).await;
            }

            msg.channel_id.say(&ctx.http, "Pong!").await;
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "";
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[derive(Debug)]
struct FeedItem {
    title: String,
    link: String,
    desc: String,
    pub_date: String,
}

// fn create_links(items: Vec<FeedItem>){
    // items
    //     .iter()
    //     .map(|item| link)
    //     .collect();
    // let mut result: Vec<FeedItem> = items;
    // if items.len() > 3 {
    //     result = items[1..3].;
    // }
    // result
// }

fn create_list(url: &str) -> Vec<FeedItem> {
    let channel = Channel::from_url(url).unwrap();
    let items: Vec<FeedItem> = channel
        .items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
            desc: item.description().unwrap().to_string(),
            pub_date: item.pub_date().unwrap().to_string(),
        })
        .collect();
    items
}