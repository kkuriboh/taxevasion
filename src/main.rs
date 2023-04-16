use poise::{serenity_prelude::*, PrefixFrameworkOptions};
use rand::Rng;

static TAXADOS_ROLE_ID: RoleId = RoleId(1092960467217023006);
// static POLYMERIA_ROLE_ID: RoleId = RoleId(935323581057994802);
// static GUILD_ID: GuildId = GuildId(935323380415082557);
static CORE_ROLE_ID: RoleId = RoleId(935325229197176922);
const ALFREDO_ID: UserId = UserId(402167442102550528);

struct Data;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping(),
                ideologia(),
                roleta(),
                reformed(),
                ban_alfredo(),
                kick_alfredo(),
                mute_alfredo(),
                mention_alfredo(),
                ban_gordola(),
            ],
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event.clone() {
                        poise::Event::GuildMemberAddition { mut new_member } => {
                            new_member.add_role(ctx.http(), TAXADOS_ROLE_ID).await?
                        }
                        poise::Event::VoiceStateUpdate { old: _, new } => {
                            let Some(mut member) = new.member else { return Ok(()) };
                            if member.user.id == ALFREDO_ID {
                                member
                                    .disable_communication_until_datetime(
                                        ctx.http(),
                                        Timestamp::from_unix_timestamp(std::i64::MAX)?,
                                    )
                                    .await?;
                            }
                        }
                        _ => (),
                    }
                    Ok(())
                })
            },
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("~>".to_string()),
                ignore_bots: true,
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .token(env!("BOT_TOKEN"))
        .intents(GatewayIntents::privileged())
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                for g in ready.guilds.iter() {
                    println!("connected to {}", Guild::get(ctx.http(), g.id).await?.name);
                }

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.say("pong").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn reformed(ctx: Context<'_>) -> Result<()> {
    let http = ctx.http();
    let author = ctx.author();

    if author
        .has_role(http, ctx.guild_id().unwrap(), CORE_ROLE_ID)
        .await?
    {
        for m in ctx
            .partial_guild()
            .await
            .unwrap()
            .members(http, None, None)
            .await?
        {
            if m.roles.contains(&TAXADOS_ROLE_ID) {
                let _ = m.ban_with_reason(http, 0, "server ta reformed").await;
            }
        }

        ctx.say("server limpo com sucesso").await?;
    } else {
        ctx.say(format!("{} {}", author.mention(), env!("REPLY_GIF_URL")))
            .await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn ban_alfredo(ctx: Context<'_>) -> Result<()> {
    let http = ctx.http();

    let alfredo = ctx
        .partial_guild()
        .await
        .unwrap()
        .member(http, ALFREDO_ID)
        .await?;

    ctx.say(format!("{} {}", alfredo.mention(), env!("REPLY_GIF_URL")))
        .await?;

    alfredo.ban_with_reason(http, 0, "alfredo").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn kick_alfredo(ctx: Context<'_>) -> Result<()> {
    let http = ctx.http();

    let alfredo = ctx
        .partial_guild()
        .await
        .unwrap()
        .member(http, ALFREDO_ID)
        .await?;

    ctx.say(format!("{} {}", alfredo.mention(), env!("REPLY_GIF_URL")))
        .await?;

    alfredo.kick_with_reason(http, "alfredo").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn mute_alfredo(ctx: Context<'_>) -> Result<()> {
    let http = ctx.http();

    let mut alfredo = ctx
        .partial_guild()
        .await
        .unwrap()
        .member(http, ALFREDO_ID)
        .await?;

    ctx.say(format!("{} {}", alfredo.mention(), env!("REPLY_GIF_URL")))
        .await?;

    alfredo
        .disable_communication_until_datetime(
            http,
            Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + 86400)?,
        )
        .await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn mention_alfredo(ctx: Context<'_>) -> Result<()> {
    let http = ctx.http();

    let alfredo = ctx
        .partial_guild()
        .await
        .unwrap()
        .member(http, ALFREDO_ID)
        .await?;

    ctx.say(format!("{} {}", alfredo.mention(), env!("REPLY_GIF_URL")))
        .await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn roleta(ctx: Context<'_>) -> Result<()> {
    let http = ctx.http();
    if let Some(guild) = ctx.partial_guild().await {
        if !ctx
            .author()
            .has_role(http, guild.clone(), CORE_ROLE_ID)
            .await?
        {
            ctx.say("não mete o dedo").await?;
            return Ok(());
        }

        let Ok(members) = guild.members(http, None, None).await else { return Ok(()) };
        let members = members
            .into_iter()
            .filter(|m| m.roles.contains(&TAXADOS_ROLE_ID))
            .collect::<Vec<_>>();

        let randomness = rand::thread_rng().gen_range(0..members.len());

        if let Some(member) = members.get(randomness) {
            member.ban_with_reason(http, 0, "você é otario").await?;
            ctx.say(format!("o macaco {} foi de berço", member.mention()))
                .await?;

            return Ok(());
        }
    }

    ctx.say("deu merda").await?;

    Ok(())
}

const GORDOLA_ID: UserId = UserId(1087936423379865714);
#[poise::command(slash_command, prefix_command)]
async fn ban_gordola(ctx: Context<'_>) -> Result<()> {
    if !ctx.author().id.eq(&UserId(911388509489745921)) {
        ctx.say("não mete o dedo").await?;
    } else if let Some(guild) = ctx.partial_guild().await {
        guild.ban(ctx.http(), GORDOLA_ID, 0).await?;
        ctx.say("presente pra você fofa").await?;
    }

    return Ok(());
}

#[poise::command(slash_command, prefix_command)]
async fn ideologia(ctx: Context<'_>) -> Result<()> {
    ctx.say("LIBERAL, FEMINISTA E APOIADOR DAS CAUSAS LGBTQIA+, TOTALMENTE CONTRA AS DROGAS, PEDOFILIA E AGRESSÃO AOS ANIMAIS.").await?;

    Ok(())
}
