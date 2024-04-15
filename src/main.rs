//Imports start here
use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::env;
// Imports end here.


struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


/* COMMANDS */

/// Hallo Welt!
#[poise::command(slash_command, prefix_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hallo Welt!").await?;
    Ok(())
}

///Informationen über den Bot
#[poise::command(slash_command, prefix_command)]
async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {

    ctx.send(poise::CreateReply::default()
    .content("Informationen über den Bot")
    .embed(serenity::CreateEmbed::new()
        .title("Bot-Info")
        .description("Bot-Version: 0.2\nVerwendete Bibliotheken: Serenity + Poise\nProgrammiersprache: Rust\nEntwickler: SvH")
    )
    .ephemeral(true)).await?;
    Ok(())
}

/// To-Do-Liste
#[poise::command(slash_command, prefix_command)]
async fn todo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default()
    .content("ToDo-Liste")
    .embed(serenity::CreateEmbed::new()
        .title("To-Do-Liste")
        .description("Embeds besser anzeigen\nBrauchbaren Hilfe-Command basteln\nCommands in seperate Files auslagern\nCommands für Memes einbauen\nNordhanar-Fact-Sheet einbauen.")
    )
    .ephemeral(true)).await?;
    Ok(())
}

/* Primary function */

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), botinfo(), todo()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId::new(931187178980126780)).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}