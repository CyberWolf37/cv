use botMessenger::{utils , api, BotMessenger};

use utils::block::Block;
use utils::block::CartBox;
use api::card::Card;
use api::card::CardGeneric;
use api::card::CardButtons;
use api::button::Button;
use api::card::DefaultAction;

fn main() {

    let TOKEN_FB = &std::env::var("TOKEN_FB").unwrap();
    let TOKEN_WH = &std::env::var("TOKEN").unwrap();
    let PORT = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    // BLOCKS
    let block_credits = Block::new("#Credits")
                        .cartBox(CartBox::new()
                            .text("Bonjour 😃 comment vas-tu ?
                                \nmoi j'ai la pêche 🎣
                                \nVeut-tu que je te détail mon CV 📖 ?
                                \nEn même temps ce n'est pas comme si je suis déstiné à faire autre chose!")
                            .button_postback("Oui 👍", "#CvStart")
                            .button_postback("Non 👎", "#Nop"));

    let block_hello = Block::new("#Credits")
                            .cartBox(CartBox::new()
                                .text("Ce Bot Messenger a étais écrit entièrement sous RUST❤️ Par Loïc Borel"))
                            .cartBox(CartBox::new()
                                .text("Veux tu revenir au menu ?")
                                .button_postback("Oui 👍", "#CvStart")
                                .button_postback("Non 👎", "#Nop"));                       

    let block_nop = Block::new("#Nop")
                        .cartBox(CartBox::new()
                            .text("Dommage je sentais une harmony 🖖 entre nous"));

    let block_cv = Block::new("#CvStart")
                        .cartBox(CartBox::new()
                            .card(CardButtons::new("Commence par choisir ce que tu veux savoir.")
                                .button(Button::new_button_pb("Sports 🏈", "#Sports"))
                                .button(Button::new_button_pb("Cursus 💼", "#Cursus"))
                                .button(Button::new_button_pb("Hobbies 💻", "#Hobbies")))
                            .card(CardButtons::new("Il y a aussi ...")
                                .button(Button::new_button_pb("Profil 😷", "#Profil"))
                                .button(Button::new_button_pb("Credits 🎓", "#Credits"))));

    let block_sports = Block::new("#Sports")
                        .cartBox(CartBox::new()
                            .text("D'aussi loin que je me souviens, le sport a toujours été un véritable plaisir pour moi"))
                        .cartBox(CartBox::new()
                            .card(CardGeneric::new("J'ai d'abord performé au Tennis")
                                .image("https://media.istockphoto.com/photos/tennis-ball-with-racket-on-the-tennis-court-sport-recreation-concept-picture-id1093339590?k=6&m=1093339590&s=612x612&w=0&h=cACZSz1kEFJIqHYBEPuq2UB4_hgZ1YfP7Kd7JbhUrlA=")
                                .subtitle("Le Tennis m'a beaucoup plus, mais il ne reflétait pas ma façon de penser"))
                            .card(CardGeneric::new("Ensuite ça a été le Volley-ball")
                                .image("https://3er1viui9wo30pkxh1v2nh4w-wpengine.netdna-ssl.com/wp-content/uploads/prod/sites/448/2018/06/FIVB-vball-1024x683.png")
                                .subtitle("Le Volley ball m'a apporté l'ésprit d'équipe"))
                            .card(CardGeneric::new("Puis j'ai eu envie de changer pour le Football Americain")
                                .image("https://frenchmorning.com/wp-content/uploads/2016/01/football-e1454340955918.jpg")
                                .subtitle("Un mélange de brutalité, de technique et de dépassement de soi"))
                            .card(CardGeneric::new("Et enfin le triathlon, car mes collègues m'y ont poussé...")
                                .image("https://tmv.tmvtours.fr/wp-content/uploads/sites/tours/AGENDA-PHOTO10-TOURSN-MAN.jpg")
                                .subtitle("ça ne va en rien avec ce que j'aimais du sport, mais j'aime ça!")))
                        .cartBox(CartBox::new()
                            .text("Veux tu revenir au menu ?")
                            .button_postback("Oui 👍", "#CvStart")
                            .button_postback("Non 👎", "#Nop"));
                            
    let block_cursus = Block::new("#Cursus")
                        .cartBox(CartBox::new()
                            .text("Je vais commencer cette section par mon entrée a l'AFORP"))
                        .cartBox(CartBox::new()
                            .card(CardGeneric::new("L'AFORP à Asnières sur seine")
                                .image("https://www.aforp.fr/formation-continue/wp-content/themes/aka_theme/images/logo-aforp.png")
                                .subtitle("J'ai commencé par être Chaudronier. J'y ai fait mon BEP et mon BAC"))
                            .card(CardGeneric::new("Ensuite j'ai Travaillé à la CETIL")
                                .image("https://lh3.googleusercontent.com/proxy/FtSNIwv1eqWTtw_TgsfH4lKv_LOv18S6MdGntH7NJxQIwTy8boRq4acUmbj11Q_iYCzHNZbJbbNDCsTiDPwiCfIAFXOGiQuJ1Tdn-xzCG2R_-dIXahv5On7vfHRaHl0")
                                .subtitle("Une entreprise que j'ai beaucoup apprécié"))
                            .card(CardGeneric::new("Après j'ai voulu faire un BTS CPI au lycée Dorian")
                                .image("https://www.lycee-dorian.fr/images/template/logo_dorian.png")
                                .subtitle("J'ai obtenu mon BTS CPI en alternance avec l'O.N.E.R.A."))
                            .card(CardGeneric::new("Après quelques autres entreprises j'ai signé pour la SEF touraine")
                                .image("https://lh3.googleusercontent.com/proxy/RoTILTvzV-I9kNHqMEGIhudQ0TTM0btMpIer8T8cMZLBCo-YDm0Wdunvn6td5q4wN9roPwKnuLF-94L8OAAOY5kDiImagnyoGD1lB4ujF54dwkI")
                                .subtitle("Je fais de la préstation chez FAIVELEY WABTEC")))
                        .cartBox(CartBox::new()
                            .text("Veux tu revenir au menu ?")
                            .button_postback("Oui 👍", "#CvStart")
                            .button_postback("Non 👎", "#Nop")); 

    let block_hobbies = Block::new("#Hobbies")
                            .cartBox(CartBox::new()
                                .text("Alors là si tu as une heure devant toi ça serait bien. Mais je vais raccourcir ma liste!"))
                            .cartBox(CartBox::new()
                                .card(CardGeneric::new("La programmation, plus qu'un Hobbie, j'en ai fait une auto-entreprise")
                                    .image("https://libreshot.com/wp-content/uploads/2016/07/programming.jpg")
                                    .subtitle("La programmation est vraiment une passion ❤️"))
                                .card(CardGeneric::new("L'électronique")
                                    .image("https://upload.wikimedia.org/wikipedia/commons/d/d9/Arduino_ftdi_chip-1.jpg")
                                    .subtitle("J'aime apprendre l'électronique en réparant les objets qui ne fonctionnent plus.🖥🔧"))
                                .card(CardGeneric::new("Le jeux vidéo")
                                    .image("https://ici.artv.ca/upload/cms/blog-content/jeux-650x366-702.jpg")
                                    .subtitle("J'aime jouer. j'espère qu'un jour j'en programmerais un.")))
                            .cartBox(CartBox::new()
                                .text("Veux tu revenir au menu ?")
                                .button_postback("Oui 👍", "#CvStart")
                                .button_postback("Non 👎", "#Nop"));
                                
    let block_profil = Block::new("#Profil")
                                .cartBox(CartBox::new()
                                    .text("En toute honnêteté et vue avec ma femme 🦄 !"))
                                .cartBox(CartBox::new()
                                    .card(CardGeneric::new("Je suis un éternel optimiste")
                                        .image("https://i1.wp.com/olivier-roland.com/wp-content/uploads/2019/11/optimisme-personne-optimiste-croire-en-soi.jpg?resize=500%2C331&ssl=1")
                                        .subtitle("“Je préfère vivre en optimiste et me tromper, que vivre en pessimiste et avoir toujours raison. ” Jean-Michel Guenassia"))
                                    .card(CardGeneric::new("Je communique ma bonne humeur")
                                        .image("data:image/png;base64,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")
                                        .subtitle("Il est plutôt rare de me voir triste"))
                                    .card(CardGeneric::new("Persévérance")
                                        .image("https://regardsprotestants.com/wp-content/uploads/2020/05/iStock-859932810.jpg")
                                        .subtitle("Ne me dites pas que je ne peux pas")))
                                .cartBox(CartBox::new()
                                    .text("Veux tu revenir au menu ?")
                                    .button_postback("Oui 👍", "#CvStart")
                                    .button_postback("Non 👎", "#Nop"));


    BotMessenger::new()
            .block(block_hello)
            .block(block_credits)
            .block(block_nop)
            .block(block_cv)
            .block(block_profil)
            .block(block_sports)
            .block(block_cursus)
            .block(block_hobbies)
            .block_default(Block::new("default")
                .cartBox(CartBox::new()
                    .text("Désoler je ne comprends pas 🐻"))
                .cartBox(CartBox::new()
                        .text("Veux tu revenir au menu ?")
                        .button_postback("Oui 👍", "#CvStart")
                        .button_postback("Non 👎", "#Nop")))
            .with_token_fb(&TOKEN_FB)
            .with_token_wh(&TOKEN_WH)
            .with_port(PORT)
            .launch();
}