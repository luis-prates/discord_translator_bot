extern crate serenity;
extern crate deepl;

use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua::Language::{English, Portuguese};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use deepl::{DeepLApi, Lang};

use serenity::framework::standard::{StandardFramework};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let languages = vec![English, Portuguese];

        let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();

        let detected_language: Option<Language> = detector.detect_language_of(&msg.content);

        let deepl = DeepLApi::with("YOUR_DEEPL_API_TOKEN_HERE").new();

        if detected_language.unwrap() == English {
            // The message is in English, translate it to Portuguese.
            let translated_text = deepl
                .translate_text(&msg.content, Lang::PT)
                .source_lang(Lang::EN)
                .await
                .unwrap();
            let translated_results = translated_text.translations;
            msg.channel_id.say(&ctx.http, &translated_results[0].text).await.unwrap();
        } else {
            // The message is in Portuguese, translate it to English.
            let translated_text = deepl
                .translate_text(&msg.content, Lang::EN)
                .source_lang(Lang::PT)
                .await
                .unwrap();
            let translated_results = translated_text.translations;
            msg.channel_id.say(&ctx.http, &translated_results[0].text).await.unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot is ready! Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "YOUR_DISCORD_BOT_TOKEN_HERE";

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")); // set the bot's prefix to "~"

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}