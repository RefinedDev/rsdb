mod commands;

use std::env;
use dotenv;
use commands::{misc, moderation};

use serenity::{
    async_trait,
    model::{

        gateway::{
            Ready,
            Activity
        },

        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommandOptionType,
            },
            Interaction,
        },
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Slash Commands Handler
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // Handle ALL The Commands
            match command.data.name.as_str() {
                "help" => misc::help(ctx, command).await,
                "ping" => misc::ping(ctx, command).await, 
                "avatar" => misc::get_avatar(ctx,command).await,
                "8ball" => misc::ball_8(ctx,command).await,
                "kick" => moderation::kick(ctx,command).await,
                "ban" => moderation::ban(ctx,command).await,
                "unban" => moderation::unban(ctx,command).await,
                _ => ()
            };
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Bot is online!");
        
        ctx.set_activity(Activity::listening("Rust")).await;

        let guild_id = GuildId(env::var("GUILD_ID")
            .expect("Expected a GUILD_ID in the environment")
            .parse::<u64>()
            .expect("The GUILD_ID needs to be an integer")
        );
        
        // Register ALL The Commands
        let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                // Help
                .create_application_command(|command| {
                    command
                        .name("help")
                        .description("List of commands")
                        .create_option(|option|{
                            option
                                .name("category")
                                .description("What type of commands do you wanna see?")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                                .add_string_choice("Moderation", "Moderation commands.")
                                .add_string_choice("Miscellaneous", "Miscellaneous commands.")
                        })
                })
                // Ping
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                // Avatar
                .create_application_command(|command| {
                    command
                        .name("avatar")
                        .description("Get the avatar of a user")
                        .create_option(|option| {
                            option
                                .name("member")
                                .description("The member you want to get the avatar of.")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)
                        })
                })
                // 8ball
                .create_application_command(|command| {
                    command
                        .name("8ball")
                        .description("Ask the magic 8ball!")
                        .create_option(|option| {
                            option
                                .name("query")
                                .description("What do you wanna ask?")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                // Kick
                .create_application_command(|command| {
                    command
                        .name("kick")
                        .description("Kick a user from the guild.")
                        .create_option(|option| {
                            option
                                .name("member")
                                .description("The member you want to get the rid of.")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)
                        })
                        .create_option(|option|{
                            option 
                                .name("reason")
                                .description("Why do you wanna get rid of them?")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                // Ban
                .create_application_command(|command| {
                    command
                        .name("ban")
                        .description("Ban a user from the guild.")
                        .create_option(|option| {
                            option
                                .name("member")
                                .description("The member you want to get the rid of.")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)
                        })
                        .create_option(|option|{
                            option 
                                .name("reason")
                                .description("Why do you wanna get rid of them?")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                //Unban
                .create_application_command(|command| {
                    command
                        .name("unban")
                        .description("Unban a user from the guild.")
                        .create_option(|option| {
                            option
                                .name("tag")
                                .description("The tag of the user you want to unban, example: Example#6969")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Load Variables From the .env File
    
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment"); 

    let application_id = env::var("APPLICATION_ID") // The APPLICATION_ID is the bot's client id.
        .expect("Expected an APPLICATION_ID in the environment")
        .parse::<u64>()
        .expect("The APPLICATION_ID needs to be an Integer.");

    let mut client = Client::builder(token) // Initialize The Bot
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Start The Bot
    if let Err(why) = client.start().await { 
        println!("Client error: {:?}", why);
    }
}