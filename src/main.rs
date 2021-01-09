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

    let block_sports = Block::new("#Sports")
                        .cartBox(CartBox::new()
                            .text("D'aussi loin que je me souviens, le sport a toujours était un veritable plaisir pour moi"))
                        .cartBox(CartBox::new()
                            .card(CardGeneric::new("J'ai d'abord performé au Tennis, car ma mère y jouait")
                                .image("https://media.istockphoto.com/photos/tennis-ball-with-racket-on-the-tennis-court-sport-recreation-concept-picture-id1093339590?k=6&m=1093339590&s=612x612&w=0&h=cACZSz1kEFJIqHYBEPuq2UB4_hgZ1YfP7Kd7JbhUrlA=")
                                .subtitle("Le Tennis m'a beaucoup plus, mais elle ne refléter pas ma façon de penser"))
                            .card(CardGeneric::new("Ensuite sa a était le Volley-ball, car mes amis y étaient inscrits")
                                .image("https://3er1viui9wo30pkxh1v2nh4w-wpengine.netdna-ssl.com/wp-content/uploads/prod/sites/448/2018/06/FIVB-vball-1024x683.png")
                                .subtitle("Le Volley ball ma apporté l'ésprit d'équipe, que je veux retrouver dans tout les jeux sportifs"))
                            .card(CardGeneric::new("Puis j'ai eu envie de changer pour le Football Americain")
                                .image("https://frenchmorning.com/wp-content/uploads/2016/01/football-e1454340955918.jpg")
                                .subtitle("Savant mélange de brutalité et de dépassement de soi avec une pincer de technique"))
                            .card(CardGeneric::new("Et enfin le triathlon, car mes collègues m'y ont poussé...")
                                .image("https://tmv.tmvtours.fr/wp-content/uploads/sites/tours/AGENDA-PHOTO10-TOURSN-MAN.jpg")
                                .subtitle("Sa ne va en rien avec ce que j'aimais du sport, mais j'aime ça!")));
                                


    BotMessenger::new()
            .block(block_hello)
            .block(block_nop)
            .block(block_cv)
            .block(block_sports)
            .block_default(Block::new("default")
                .cartBox(CartBox::new()
                    .text("Désoler je ne comprends pas 🐻")))
            .with_token_fb(&TOKEN_FB)
            .with_token_wh(&TOKEN_WH)
            .with_port(PORT)
            .launch();
}