use botMessenger::{utils , api, BotMessenger};

use utils::block::Block;
use utils::block::CartBox;
use api::card::Card;
use api::card::CardGeneric;
use api::card::CardButton;
use api::button::Button;

fn main() {

    let TOKEN_FB = &std::env::var("TOKEN_FB").unwrap_or("MamaGuriba".to_string());
    let TOKEN_WH = &std::env::var("TOKEN").unwrap_or("MamaGuriba".to_string());
    let PORT = std::env::var("PORT").unwrap_or("7878".to_string()).parse::<u16>().unwrap_or(7878);

    // BLOCKS
    let block_hello = Block::new("#Start")
                        .cartBox(CartBox::new()
                            .text("Bonjour 😃 comment vas-tu ?
                                \nmoi j'ai la pêche 🎣
                                \nVeut-tu que je te détail mon CV 📖 ?")
                            .button_postback("Oui 👍", "#CvStart")
                            .button_postback("Non 👎", "#Nop"));

    let block_nop = Block::new("#Nop")
                        .cartBox(CartBox::new()
                            .text("Dommage je sentais une harmony 🖖 entre nous"));

    let block_cv = Block::new("#CvStart")
                        .cartBox(CartBox::new()
                            .card(CardButton::new()
                                .button(Button::new_button_pb("Sports", "#Sports"))
                                .button(Button::new_button_pb("Cursus", "#Cursus")
                                .button(Button::new_button_pb("Hobbies", "#Hobbies")))));

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