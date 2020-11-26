use botMessenger::{utils , api, BotMessenger};

use utils::{Block,CartBox,BotUser};
use api::{Message,ApiMessage};
use std::sync::Arc;
use log::*;

fn main() {
    let mut bot = BotMessenger::new();
    bot.get_conf_mut().set_token_fb_page(&std::env::var("TOKEN_FB").unwrap_or("MamaGuriba".to_string()));
    bot.get_conf_mut().set_token_webhook(&std::env::var("TOKEN").unwrap_or("MamaGuriba".to_string()));
    
    let mut block = Block::new("Hello");
        block.add(Arc::new(CartBox::new(Arc::new(|x: &BotUser| {
           let mes = Message::new("Hello mother fucker");
           mes.send(x);
        }))));
        bot.add_block(block);
        println!("{}",bot.get_conf());
        bot.launch();
}
