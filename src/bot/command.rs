use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "list tracked keywords.")]
    Keywords,
    #[command(description = "add a domain keyword to track.")]
    Track(String),
    #[command(description = "get the current chat id.")]
    ChatId,
}
