use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::env;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Simple Hello World command. 
#[poise::command(slash_command, prefix_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello World!").await?;
    Ok(())
}

/// Another "Test-Command"
#[poise::command(slash_command, prefix_command)]
async fn lorem(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Lorem Ipsum dolor sit amet!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Bot-Version: 0.1\nVerwendete Bibliotheken: Serenity + Poise\nProgrammiersprache: Rust\nEntwickler: SvH").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), hello(), lorem(), botinfo()],
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