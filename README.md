A discord bot made using [Serenity](https://github.com/serenity-rs/serenity).<br>
[Documentation](https://docs.rs/serenity) for the library.<br>

To add commands edit the `main.rs` and use the `.create_application_command()` method to add more commands.<br>
Each category of commands have their own file in the `commands` directory.<br>

**Also add your things in the .env file.**<br>
BOT_TOKEN, Your bot's token<br>

GUILD_ID, The guild you will be testing your bot (need this because slash commands take an hour or so to load to guilds but if you provide a guild explicitly then it loads there instantly.)<br>

APPLICATION_ID, The bot's client id.
