use botMessenger::{utils , api, BotMessenger};

use utils::block::Block;
use utils::block::CartBox;
use api::card::Card;
use api::card::CardGeneric;
use api::card::CardButtons;
use api::button::Button;

fn main() {

    let TOKEN_FB = &std::env::var("TOKEN_FB").unwrap();
    let TOKEN_WH = &std::env::var("TOKEN").unwrap();
    let PORT = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    // BLOCKS
    let block_hello = Block::new("#Start")
                        .cartBox(CartBox::new()
                            .text("Bonjour 😃 comment vas-tu ?
                                \nmoi j'ai la pêche 🎣
                                \nVeut-tu que je te détail mon CV 📖 ?
                                \nEn même temps ce n'est pas comme si je suis déstiné à faire autre chose!")
                            .button_postback("Oui 👍", "#CvStart")
                            .button_postback("Non 👎", "#Nop"));

    let block_nop = Block::new("#Nop")
                        .cartBox(CartBox::new()
                            .text("Dommage je sentais une harmony 🖖 entre nous"));

    let block_cv = Block::new("#CvStart")
                        .cartBox(CartBox::new()
                            .card(CardButtons::new("Alors commence par choisir ce que tu veux savoir.")
                                .button(Button::new_button_pb("Sports 🏈", "#Sports"))
                                .button(Button::new_button_pb("Cursus 💼", "#Cursus"))
                                .button(Button::new_button_pb("Hobbies 💻", "#Hobbies"))));


    BotMessenger::new()
            .block(block_hello)
            .block(block_nop)
            .block(block_cv)
            .block_default(Block::new("default")
                .cartBox(CartBox::new()
                    .text("Sorry I don't understand 🐻")))
            .with_token_fb(&TOKEN_FB)
            .with_token_wh(&TOKEN_WH)
            .with_port(PORT)
            .launch();
}