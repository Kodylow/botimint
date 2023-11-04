# Botimint: the Fedimint Development Bot

This project is a Fedimint development bot that uses the Fedimint client. It is built using the Serenity framework for Discord bots. The bot is designed to interact with users on the Fedimint Discord server, providing various commands and responses to Fedimint related things.

To run this bot, you will need a valid Discord Token. Follow these steps to get started:

1. Log in to the Discord developer portal.
2. Click the New Application button, name your application, and click Create.
3. Navigate to the Bot tab in the left-hand menu, and add a new bot.
4. On the bot page, click the Reset Token button to reveal your token. Store this token in your .env file. Be careful not to reveal your token to anyone, as it can be misused.

To add the bot to a server, create an invite link:

1. On your bot's application page, open the OAuth2 page via the left-hand panel.
2. Go to the URL Generator via the left-hand panel, select the bot scope, and the Send Messages permission in the Bot Permissions section.
3. Copy the URL, open it in your browser, and select a Discord server to invite the bot to.

The bot's functionality is defined in the src/commands directory. Each command is defined in its own file, such as src/commands/welcome.rs for the welcome command.

For more information, please refer to the Discord docs and the Serenity repo for more examples.
