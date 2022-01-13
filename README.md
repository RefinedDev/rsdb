A discord bot made using [Serenity](https://github.com/serenity-rs/serenity).
[Documentation](https://docs.rs/serenity) for the library.

To add commands edit the `main.rs` and use the `.create_application_command()` method to add more commands.
Each category of commands have their own file in the `commands` directory.

**Also add your things in the .env file.**
BOT_TOKEN, Your bot's token

GUILD_ID, The guild you will be testing your bot (need this because slash commands take an hour or so to load to guilds but if you provide a guild explicitly then it loads there instantly.)

APPLICATION_ID, The bot's client id.
