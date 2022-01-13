use serenity::{
    model::{
        interactions::{
            application_command::{
                ApplicationCommandInteraction,
                ApplicationCommandInteractionDataOptionValue,
            },

            InteractionResponseType,
        },
    },

    prelude::*,
};

use rand::seq::SliceRandom;

pub async fn help(ctx: Context, command: ApplicationCommandInteraction) {
    let chosen_category = command.data.options.get(0).expect("Expected category option").resolved.as_ref().expect("Expected category object");
    
    if let ApplicationCommandInteractionDataOptionValue::String(value) =    
        chosen_category {
            let mut title: String = "hi".to_string();
            let mut category_commands: Vec<&str> = vec!["hye"];

            // LENGTHEN THIS IF SEQUENCE WHEN YOU ADD MORE CATEGORIES
            if value == "Moderation commands." {
                title = "Moderation Commands".to_string();
                category_commands = vec!["kick", "ban", "unban"]; // EDIT THIS AS YOU ADD MORE COMMANDS
            }else if value == "Miscellaneous commands." {
                title = "Miscellaneous Commands".to_string();
                category_commands = vec!["ping", "getavatar", "8ball"]; // EDIT THIS AS YOU ADD MORE COMMANDS
            }

            if let Err(why) = command.create_interaction_response(&ctx.http, |response|{
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message|{
                        message.create_embed(|e|{
                            e.title(title);
                            
                           for i in category_commands.iter() { 
                              e.field(i, format!("/{}", i), false);
                           }
        
                            e
                        });
        
                        message
                    })
            }).await
        
            { println!("Cannot respond to slash command: {}", why); }
        }
}

pub async fn ping(ctx: Context, command: ApplicationCommandInteraction) {
    // Respond To The Message
    if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Pong"))
    }).await
    
    // If Failed
    { println!("Cannot respond to slash command: {}", why); }
}

pub async fn get_avatar(ctx: Context, command: ApplicationCommandInteraction) {
    let member = command.data.options.get(0).expect("Expected user option").resolved.as_ref().expect("Expected user object");

    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = 
        member {
            let hash = match user.avatar {
               None => "404",
               Some(ref x) => x,
            };

            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                if hash == "404" {
                    response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("Unable to fetch avatar from user."))
                }else{
                    response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.create_embed(|e|{
                                e.title(format!("{}'s Avatar", user.name));
                                e.description(format!("**Links**\n[PNG](https://cdn.discordapp.com/avatars/{}/{}.png?size=1024) | [JPG](https://cdn.discordapp.com/avatars/{}/{}.jpg?size=1024) | [WEBP](https://cdn.discordapp.com/avatars/{}/{}.webp?size=1024)",user.id, hash, user.id, hash, user.id, hash));
                                e.image(format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=1024",user.id, hash));
                        
                                e
                            });

                            message
                        })
                }      
            }).await

            { println!("Cannot respond to slash command: {}", why); }
        }
}

pub async fn ball_8(ctx: Context, command: ApplicationCommandInteraction) {
    let answers = vec!["As I see it, yes.","Ask again later.","Better not tell you now.","Cannot predict now.","Concentrate and ask again.","Don't count on it.","It is certain.","It is decidedly so.","Most likely.","My reply is no.",
    "My sources say no.","Outlook not so good.","Outlook good.","Reply hazy, try again.","Signs point to yes.","Very doubtful.","Without a doubt.","Yes.","Yes, definitely.","You may rely on it."];

    let answer = answers.choose(&mut rand::thread_rng()).expect("Unable to choose 8ball answer.");

    if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(answer))
    }).await
    
    // If Failed
    { println!("Cannot respond to slash command: {}", why); }
}
