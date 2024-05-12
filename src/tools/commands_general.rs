use poise::serenity_prelude as serenity;


pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Hallo Welt!
#[poise::command(slash_command, prefix_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hallo Welt!").await?;
    Ok(())
}

///Informationen über den Bot
#[poise::command(slash_command, prefix_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {

    ctx.send(poise::CreateReply::default()
    .content("Informationen über den Bot")
    .embed(serenity::CreateEmbed::new()
        .title("Bot-Info")
        .description("Bot-Version: 0.3\nVerwendete Bibliotheken: Serenity + Poise\nProgrammiersprache: Rust\nEntwickler: SvH")
    )
    .ephemeral(true)).await?;
    Ok(())
}

/* GIFs */
// Atombombenfetisch
#[poise::command(slash_command, prefix_command)]
pub async fn atombombenfetisch(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://c.tenor.com/4dPZgUkQcaQAAAAd/tenor.gif").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn sirene(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://www.youtube.com/watch?v=54ODp_Ypx_E").await?;
    Ok(())
}

/// To-Do-Liste
#[poise::command(slash_command, prefix_command)]
pub async fn todo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default()
    .content("ToDo-Liste")
    .embed(serenity::CreateEmbed::new()
        .title("To-Do-Liste")
        .description("Embeds besser anzeigen\nBrauchbaren Hilfe-Command basteln\nCommands für Memes einbauen\nNordhanar-Fact-Sheet einbauen.")
    )
    .ephemeral(true)).await?;
    Ok(())
}