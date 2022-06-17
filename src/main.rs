use rand::Rng;

fn main() {
    let mut arcanes_majeurs: Vec<&str> = vec!["Le Fou", "Le Magicien", "La Grande Prêtresse", "L'Impératrice", "L'Empereur", "Le Hiérophant", "Les Amoureux", "Le Chariot", "La Force", "L'Hermite", "La Roue de Fortune", "La Justice", "Le Pendu", "La Mort", "La Tempérance", "Le Diable", "La Tour", "L'Étoile", "La Lune", "Le Soleil", "Le Jugement", "Le Monde"];
    let mut cards: Vec<&str> = vec![];

    for i in 0..=2 {
        let random_card = rand::thread_rng().gen_range(0..(22 - i));
        cards.push(arcanes_majeurs[random_card]);
        arcanes_majeurs.remove(random_card);
    }

    println!("{:?}", cards);
}
