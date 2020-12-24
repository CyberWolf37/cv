use botMessenger::{utils , api, BotMessenger};

use utils::block::Block;
use utils::block::CartBox;
use api::card::Card;
use api::card::CardGeneric;
use api::button::Button;

fn main() {

    let TOKEN_FB = &std::env::var("TOKEN_FB").unwrap_or("MamaGuriba".to_string());
    let TOKEN_WH = &std::env::var("TOKEN").unwrap_or("MamaGuriba".to_string());
    let PORT = std::env::var("PORT").unwrap_or("7878".to_string()).parse::<u16>().unwrap_or(7878);

    BotMessenger::new()
            .block(Block::new("Hello")
                .cartBox(CartBox::new()
                    .text("Hello new user"))
                .cartBox(CartBox::new()
                    .text("It's a new day")
                    .button_postback("Push", "Hello"))
                .cartBox(CartBox::new()
                    .card(CardGeneric::new("Hello")
                        .button(Button::new_button_pb("Welcom back Mr potter", "Hello"))
                        .image("https://images.ladepeche.fr/api/v1/images/view/5c34fb833e454650457f60ce/large/image.jpg")
                        .subtitle("Bouyah"))))
            .block(Block::new("#Start")
                .cartBox(CartBox::new()
                    .text("New start user")))
            .with_token_fb(&TOKEN_FB)
            .with_token_wh(&TOKEN_WH)
            .with_port(PORT)
            .launch();
}