use botMessenger::{utils , api, BotMessenger};

use utils::block::Block;
use utils::block::CartBox;
use api::card::Card;
use api::card::CardGeneric;
use api::card::CardButtons;
use api::button::Button;
use api::card::DefaultAction;
use std::path::Path;
use std::env;

fn main() {

    let TOKEN_FB = &std::env::var("TOKEN_FB").unwrap();
    let TOKEN_WH = &std::env::var("TOKEN").unwrap();
    let PORT = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    // BLOCKS
    let block_credits = Block::new("#Start")
                        .cartBox(CartBox::new()
                            .text("Bonjour 😃 comment vas-tu ?
                                \nmoi j'ai la pêche 🎣
                                \nVeut-tu que je te détail mon CV 📖 ?
                                \nEn même temps ce n'est pas comme si je suis déstiné à faire autre chose!")
                            .button_postback("Oui 👍", "#CvStart")
                            .button_postback("Non 👎", "#Nop"));

    let block_hello = Block::new("#Credits")
                            .cartBox(CartBox::new()
                                .text("Ce Bot Messenger a été écrit entièrement sous RUST🦀 Par Loïc Borel"))
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
                                .button(Button::new_button_pb("Hobbies 💻", "#Hobbies"))))
                        .cartBox(CartBox::new()
                            .card(CardButtons::new("Il y a aussi ...")
                                .button(Button::new_button_pb("Profil 😷", "#Profil"))
                                .button(Button::new_button_pb("Contact 📬", "#Contact"))
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
                                .image("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRkRjs04UnTgGkbN3EHEI_ma7pGXUvvDz0BiQ&usqp=CAU")
                                .subtitle("Une entreprise que j'ai beaucoup apprécié"))
                            .card(CardGeneric::new("Après j'ai voulu faire un BTS CPI au lycée Dorian")
                                .image("https://www.lycee-dorian.fr/images/template/logo_dorian.png")
                                .subtitle("J'ai obtenu mon BTS CPI en alternance avec l'O.N.E.R.A."))
                            .card(CardGeneric::new("Après quelques autres entreprises j'ai signé pour la SEF touraine depuis 2016")
                                .image("https://lh3.googleusercontent.com/proxy/skEQK9qDvJaOa0PyZDMkUmnGJsWuxWqHOMZHkzUbxGdU1CRu4UK1_Ewq0vlsD1tm8oEVuzR8XNkcj_Ypk3KPoANX2U57f4Z1WggfV2xSxMkxUug")
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
                                .card(CardGeneric::new("La lecture")
                                    .image("https://france3-regions.francetvinfo.fr/image/qKSRhbJH9Rj-agcqIEqXTjFlQeE/1200x900/regions/2020/06/09/5edf6e0648d98_nuit_de_la_lecture-4042071.jpg")
                                    .subtitle("J'aime lire. Surtout dans le domaine de la fantasy."))
                                .card(CardGeneric::new("Le jeux vidéo")
                                    .image("https://ici.artv.ca/upload/cms/blog-content/jeux-650x366-702.jpg")
                                    .subtitle("J'aime jouer. j'espère qu'un jour j'en programmerais un.")))
                            .cartBox(CartBox::new()
                                .text("Veux tu revenir au menu ?")
                                .button_postback("Oui 👍", "#CvStart")
                                .button_postback("Non 👎", "#Nop"));
                                
    let block_profil = Block::new("#Profil")
                                .cartBox(CartBox::new()
                                    .text("En toute honnêteté et vu avec ma femme ❤️ !"))
                                .cartBox(CartBox::new()
                                    .card(CardGeneric::new("Je suis un éternel optimiste")
                                        .image("http://ressources-plurielles.com/wp-content/uploads/2016/01/optimisme-ressources-plurielles-2.jpg")
                                        .subtitle("L'optimisme, c'est voir les problèmes et croire aux remèdes. Laurent Roman"))
                                    .card(CardGeneric::new("Je communique ma bonne humeur")
                                        .image("http://ekladata.com/9OqLSWMIM5jYTIG5JZOlTVkrCBo@550x688.jpg")
                                        .subtitle("Il est plutôt rare de me voir triste"))
                                    .card(CardGeneric::new("Persévérance")
                                        .image("https://regardsprotestants.com/wp-content/uploads/2020/05/iStock-859932810.jpg")
                                        .subtitle("Ne me dites pas que je ne peux pas")))
                                .cartBox(CartBox::new()
                                    .text("Veux tu revenir au menu ?")
                                    .button_postback("Oui 👍", "#CvStart")
                                    .button_postback("Non 👎", "#Nop"));

    let block_contact = Block::new("#Contact")
                                .cartBox(CartBox::new()
                                    .text("Voici mon e-mail borel.loic.37@gmail.com 📬"))
                                .cartBox(CartBox::new()
                                    .text("Veux tu revenir au menu ?")
                                    .button_postback("Oui 👍", "#CvStart")
                                    .button_postback("Non 👎", "#Nop"));

    let mut static_path = "./static";

    BotMessenger::new()
            .block(block_hello)
            .block(block_credits)
            .block(block_nop)
            .block(block_cv)
            .block(block_profil)
            .block(block_sports)
            .block(block_cursus)
            .block(block_contact)
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
<<<<<<< HEAD
            .with_static_file("./static")
=======
            .with_static_file(static_path)
>>>>>>> 2757fa088253cc81761daeee98d9c52ac609b746
            .launch();
            
}