use serenity::{
    model::{
        Permissions,
        interactions::{
            application_command::{
                ApplicationCommandInteraction,
                ApplicationCommandInteractionDataOptionValue,
            },
        
            InteractionResponseType,
        },
    },

    utils::{
        Colour
    },

    prelude::*,
};


pub async fn kick(ctx: Context, command: ApplicationCommandInteraction) {
    let member = command.data.options.get(0).expect("Expected user option").resolved.as_ref().expect("Expected user object");
    let reason = command.data.options.get(1).expect("Expected reason option").resolved.as_ref().expect("Expected reason string");

    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = 
        member {
            let guild_id = *command.guild_id.expect("Unable to fetch guild id!").as_u64();
            let member_to_kick = ctx.http.get_member(guild_id, *user.id.as_u64()).await;
            
            let mut resultant_color = Colour::from_rgb(255, 0, 0);
            let result: String;
            
            let author = &command.member;
            let permissions = 
                match author {
                    Some(x) => x.permissions.expect("Unable to fetch"),
                    None => Permissions {bits: 0}
                };   

           
            if (permissions != Permissions {bits: 0}) && (permissions.kick_members()) {
                match  member_to_kick {
                    Ok(v) => {
                        if let ApplicationCommandInteractionDataOptionValue::String(value) = 
                            reason {
                                match v.kick_with_reason(&ctx.http, value).await {
                                    Ok(()) => { 
                                        resultant_color = Colour::from_rgb(0, 255, 0);
                                        result = format!("Successfully kicked member: `{}`\nReason: `{}`", user.tag(),value);
                                    },
    
                                    Err(serenity::Error::Model(ModelError::GuildNotFound)) => {result =  "Couldn't determine guild of member".to_string();}
                                    Err(serenity::Error::Http(_)) =>  {result = format!("I Don't have permissions; missing: `{:?}`", "kick_members");}
                                    Err(v) => { result = format!(" Error: {}", v); },
                                }
                            }else {
                                result = "Reason was not set correctly, try again.".to_string();
                            }
                    },
    
                    Err(_) => {result = "Unable to fetch user!".to_string();}
                };
            }else {
                result = "You need permissions; `kick_members`".to_string();
            }
           
            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.create_embed(|e|{
                            e.description(format!("{}", result));
                            e.color(resultant_color);
                    
                            e
                        });

                        message
                    })
            }).await
        
             // If Nothing Works...
            { println!("Cannot respond to slash command: {}", why); }
        }
}

pub async fn ban(ctx: Context, command: ApplicationCommandInteraction) {
    let member = command.data.options.get(0).expect("Expected user option").resolved.as_ref().expect("Expected user object");
    let reason = command.data.options.get(1).expect("Expected reason option").resolved.as_ref().expect("Expected reason string");

    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = 
        member {
            let guild_id = *command.guild_id.expect("Unable to fetch guild id!").as_u64();
            
            let member_to_kick = ctx.http.get_member(guild_id, *user.id.as_u64()).await;
            
            let mut resultant_color = Colour::from_rgb(255, 0, 0);
            let result: String;

            let author = &command.member;
            let permissions = 
                match author {
                    Some(x) => x.permissions.expect("Unable to fetch"),
                    None => Permissions {bits: 0}
                };   
            
            if (permissions != Permissions {bits: 0}) && (permissions.ban_members()) {
                match  member_to_kick {
                    Ok(v) => {
                        if let ApplicationCommandInteractionDataOptionValue::String(value) = 
                            reason {
                                match v.ban_with_reason(&ctx.http,7,value).await {
                                    Ok(()) => { 
                                        resultant_color = Colour::from_rgb(0, 255, 0);
                                        result = format!("Successfully banned member: `{}`\nReason: `{}`", user.tag(),value);
                                    },
    
                                    Err(serenity::Error::Model(ModelError::GuildNotFound)) => {result =  "Couldn't determine guild of member".to_string();}
                                    Err(serenity::Error::Http(_)) =>  {result = format!("I don't have permissions; missing: `{:?}`", "ban_members");}
                                    Err(v) => { result = format!(" Error: {}", v); },
                                }
                            }else {
                                result = "Reason was not set correctly, try again.".to_string();
                            }
                    },
    
                    Err(_) => {result = "Unable to fetch user!".to_string();}
                };
            }else {
                result = "You need permissions; `ban_members`".to_string();
            }

            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.create_embed(|e|{
                            e.description(format!("{}", result));
                            e.color(resultant_color);
                    
                            e
                        });

                        message
                    })
            }).await
        
             // If Nothing Works...
            { println!("Cannot respond to slash command: {}", why); }
        }
}

pub async fn unban(ctx: Context, command: ApplicationCommandInteraction) {
    let id = command.data.options.get(0).expect("Expected id option").resolved.as_ref().expect("Expected id object");

    if let ApplicationCommandInteractionDataOptionValue::String(value) =
        id {
            let guild_id = *command.guild_id.expect("Unable to fetch guild id!").as_u64();

            let mut result: String = "The user is not banned!".to_string();
            let mut resultant_color = Colour::from_rgb(255, 0, 0);
            let guild = ctx.http.get_guild(guild_id).await;

            let author = &command.member;
            let permissions = 
                match author {
                    Some(x) => x.permissions.expect("Unable to fetch"),
                    None => Permissions {bits: 0}
                };   
            
            if (permissions != Permissions {bits: 0}) && (permissions.ban_members()) {
                match guild {
                    Ok(v) => {
                        match v.bans(&ctx.http).await {
                            Ok(ban_list) => {
                                for i in ban_list.iter() {
                                    if i.user.tag() == *value {
                                        match v.unban(&ctx.http, i.user.id).await {
                                            Ok(_) => {
                                                resultant_color = Colour::from_rgb(0, 255, 0);
                                                result = format!("Sucessfully unbanned, {}", i.user.tag());
                                            }
    
                                            Err(v) => {result = format!("Couldn't unban user: {}", v)}
                                        }
                                    }
                                }
                            }
                            
                            Err(serenity::Error::Http(_)) =>  {result = format!("I don't have permissions; missing: `{:?}`", "ban_members");}
                            Err(e) => {result = format!("An error occured fetching the bans: {}", e)}
                        }
                    }
    
                    Err(e) => {result = format!("An error occured: {}", e)}
                }
            }else {
                result = "You need permissions; `ban_members`".to_string();
            }
            

            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.create_embed(|e|{
                            e.description(format!("{}", result));
                            e.color(resultant_color);
                    
                            e
                        });

                        message
                    })
            }).await
        
             // If Nothing Works...
            { println!("Cannot respond to slash command: {}", why); }
        }
}