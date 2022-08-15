use std::error::Error;
use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText,
    },
    utils::command::BotCommands,
};

#[derive(BotCommands)]
#[command(
    rename = "lowercase",
    description = "These are the commands for this bot: "
)]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Start this bot")]
    Start,
    #[command(description = "Check In")]
    CheckIn,
    #[command(description = "Check Out")]
    CheckOut,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init(); // Need to export TELOXIDE_TOKEN=5798757345:AAEryOQmaspsuctg56H1_svQY0aGpyaAnd4
    log::info!("Starting the notification bot ...");

    let bot = Bot::from_env().auto_send();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

// Create a keyboard made by buttons in a big columns.
fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let bot_command = ["Help", "Check In", "Check Out"];

    for commands in bot_command.chunks(2) {
        let row = commands
            .iter()
            .map(|&commands| {
                InlineKeyboardButton::callback(commands.to_owned(), commands.to_owned())
            })
            .collect();
        keyboard.push(row)
    }
    InlineKeyboardMarkup::new(keyboard)
}

// fn make_names()->String {
//     let name = ["Abi", "Abu", "Ali", "Badrul", "Borhan", "Bedol"];

// }

// Parse the text wrote on Telegram and check if that is a valid command
// or not, then march the command. If the command is `/Start`, it writes a
// markup with the `InlineKeyboardMarkup`.
async fn message_handler(
    m: Message,
    bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = m.text() {
        match BotCommands::parse(text, "buttons") {
            Ok(Command::Help) => {
                // Send the description of all commands.
                bot.send_message(m.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Start) => {
                // Create a list of buttons and send them.
                let keyboard = make_keyboard();
                bot.send_message(m.chat.id, "Update: ")
                    .reply_markup(keyboard)
                    .await?;
            }
            Ok(Command::CheckIn) => {
                // Create a list of buttons and send mock status.
                let keyboard = make_keyboard();
                bot.send_message(m.chat.id, "Some people has check in")
                    .reply_markup(keyboard)
                    .await?;
            }
            Ok(Command::CheckOut) => {
                // Create a list of buttons and send mock status.
                let keyboard = make_keyboard();
                bot.send_message(m.chat.id, "Some people has check out ")
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(_) => {
                bot.send_message(
                    m.chat.id,
                    "Command not found! Please try again or type '/Help'",
                )
                .await?;
            }
        }
    }
    Ok(())
}

async fn inline_query_handler(
    q: InlineQuery,
    bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let choose_command = InlineQueryResultArticle::new(
        "0",
        "Choose command",
        InputMessageContent::Text(InputMessageContentText::new("Update: ")),
    )
    .reply_markup(make_keyboard());

    bot.answer_inline_query(q.id, vec![choose_command.into()])
        .await?;

    Ok(())
}

async fn callback_handler(
    q: CallbackQuery,
    bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(commands) = q.data {
        let text = format!("Today's update is: {:?} ", commands);

        match q.message {
            Some(Message { id, chat, .. }) => {
                bot.edit_message_text(chat.id, id, text).await?;
            }
            None => {
                if let Some(id) = q.inline_message_id {
                    bot.edit_message_text_inline(id, text).await?;
                }
            }
        }
        log::info!("Today's update is: {:?}", commands);
    }
    Ok(())
}
