# THIS IS A TELEGRAM NOTIFICATION BOT

This bot is use for the project in which it will give telegram notification to user about things.

### TO RUN THIS PROGRAM WITH OPTIMIZATION

`cargo run --release --bin tele_notification`

### SIMPLY TO RUN

`cargo run main.rs`

or

`cargo run .`

### TO BUILD WITH OPTIMIZATION

`cargo build --release --bin tele_notification`

### SIMPLY TO BUILD

`cargo build main.rs`

or

`cargo build`

- This program will be use for the future project. Right now it is just a POC for the client
- Need to find a server that will run 24/7 to maintain the service for the telegram bot
- Use Teloxide for the telegram bot API
- Don't have any idea yet on how to make this program. Maybe for the POC, can just show a simple conversation
- Need to figure out how can the bot start the conversation. According to the official docs, bots cannot start the conversation. Maybe can make it spit out something by linking with the API for notification? **_MAYBE_**
- **_IMPORTANT_** Do not share this with anyone, but who cares.

### ISSUES AND IMPROVEMENTS
- There is some bug at check in and check out part
- Need to make database for the "check in" and "check out" function
- Connect to the existing API so that we can generate the things we want 
