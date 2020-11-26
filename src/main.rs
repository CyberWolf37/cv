use botMessenger::{utils , api, BotMessenger};

use utils::{Block,CartBox,BotUser};
use api::{Message,ApiMessage};
use std::sync::Arc;
use log::*;

fn main() {
    let mut bot = BotMessenger::new();
    bot.get_conf_mut().set_token_fb_page(&std::env::var("TOKEN_FB").unwrap_or("MamaGuriba".to_string()));
    bot.get_conf_mut().set_token_webhook(&std::env::var("TOKEN").unwrap_or("MamaGuriba".to_string()));
    bot.get_conf_mut().set_port(std::env::var("PORT").unwrap_or("7878".to_string()).parse::<u16>().unwrap_or(7878));

    let message = Message::new(bot.get_conf().get_token_fb_page());
    let message_1 = Message::new(bot.get_conf().get_token_fb_page());

    let mut block = Block::new("Hello");
        block.add(Arc::new(CartBox::new(Arc::new(move |x: &BotUser| {
            message.send(x,"Hello mother fucker");
        }))));
    let block_start = Block::new("#start");
        block.add(Arc::new(CartBox::new(Arc::new(move |x: &BotUser| {
            message_1.send(x,"Start cv");
        }))));
        bot.add_block(block)
           .add_block(block_start);
        println!("{}",bot.get_conf());
        bot.launch();
}