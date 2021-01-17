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
                                .subtitle("Sa ne va en rien avec ce que j'aimais du sport, mais j'aime ça!")))
                        .cartBox(CartBox::new()
                            .text("Veux tu revenir au menu ?")
                            .button_postback("Oui 👍", "#CvStart")
                            .button_postback("Non 👎", "#Nop"));
                            
    let block_cursus = Block::new("#Cursus")
                        .cartBox(CartBox::new()
                            .text("Je vais commencer cette sections par mon entrée a l'AFORP"))
                        .cartBox(CartBox::new()
                            .card(CardGeneric::new("L'AFORP à Asnières sur seine")
                                .image("https://www.aforp.fr/formation-continue/wp-content/themes/aka_theme/images/logo-aforp.png")
                                .subtitle("J'ai commencer par être Chaudronier. J'y ai fait mon BEP et mon BAC"))
                            .card(CardGeneric::new("Ensuite J'ai Travailler a la CETIL")
                                .image("https://3er1viui9wo30pkxh1v2nh4w-wpengine.netdna-ssl.com/wp-content/uploads/prod/sites/448/2018/06/FIVB-vball-1024x683.png")
                                .subtitle("Une entreprise que j'ai beaucoup apprécier"))
                            .card(CardGeneric::new("Après j'ai voulu faire un BTS CPI au lycée Dorian")
                                .image("https://www.lycee-dorian.fr/images/template/logo_dorian.png")
                                .subtitle("J'ai obtenue mon BTS CPI en alternance avec l'O.N.E.R.A."))
                            .card(CardGeneric::new("Après quelques autres entreprise j'ai signée pour la SEF touraine")
                                .image("https://lh3.googleusercontent.com/proxy/RoTILTvzV-I9kNHqMEGIhudQ0TTM0btMpIer8T8cMZLBCo-YDm0Wdunvn6td5q4wN9roPwKnuLF-94L8OAAOY5kDiImagnyoGD1lB4ujF54dwkI")
                                .subtitle("Je fait de la préstation chez FAIVELEY WABTEC")))
                        .cartBox(CartBox::new()
                            .text("Veux tu revenir au menu ?")
                            .button_postback("Oui 👍", "#CvStart")
                            .button_postback("Non 👎", "#Nop")); 

    let block_hobbies = Block::new("#Hobbies")
                            .cartBox(CartBox::new()
                                .text("Alors là si tu as une heure devant toi sa serais bien. Mais je vais raccourcir ma liste!"))
                            .cartBox(CartBox::new()
                                .card(CardGeneric::new("La programmation, plus qu'un Hobbie, j'en ai fait une auto-entreprise")
                                    .image("https://libreshot.com/wp-content/uploads/2016/07/programming.jpg")
                                    .subtitle("La programmation est vraiment une passion ❤️"))
                                .card(CardGeneric::new("L'électronique")
                                    .image("https://upload.wikimedia.org/wikipedia/commons/d/d9/Arduino_ftdi_chip-1.jpg")
                                    .subtitle("J'aime apprendre l'électronique en reparant les objets qui ne fonctionne plus.🖥🔧"))
                                .card(CardGeneric::new("Le jeux vidéo")
                                    .image("https://ici.artv.ca/upload/cms/blog-content/jeux-650x366-702.jpg")
                                    .subtitle("J'aime jouer. j'espère que un jour j'en programmerais un.")))
                            .cartBox(CartBox::new()
                                .text("Veux tu revenir au menu ?")
                                .button_postback("Oui 👍", "#CvStart")
                                .button_postback("Non 👎", "#Nop")); 


    BotMessenger::new()
            .block(block_hello)
            .block(block_nop)
            .block(block_cv)
            .block(block_sports)
            .block(block_cursus)
            .block(block_hobbies)
            .block_default(Block::new("default")
                .cartBox(CartBox::new()
                    .text("Désoler je ne comprends pas 🐻")))
            .with_token_fb(&TOKEN_FB)
            .with_token_wh(&TOKEN_WH)
            .with_port(PORT)
            .launch();
}