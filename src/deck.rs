use rand::Rng;

#[derive(Debug, Clone)]
pub struct Card(pub usize, pub String);

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = vec![];
        let arcanes_majeurs: Vec<&str> = vec!["Le Fou", "Le Magicien", "La Grande Prêtresse", "L'Impératrice", "L'Empereur", "Le Hiérophant", "Les Amoureux", "Le Chariot", "La Force", "L'Hermite", "La Roue de Fortune", "La Justice", "Le Pendu", "La Mort", "La Tempérance", "Le Diable", "La Tour", "L'Étoile", "La Lune", "Le Soleil", "Le Jugement", "Le Monde"];

        for i in 0..arcanes_majeurs.len() {
            cards.push(Card(i, arcanes_majeurs[i].to_string()));
        }

        Deck {
            cards,
        }
    }

    pub fn take_card(&mut self) -> Card {
        let index = rand::thread_rng().gen_range(0..self.cards.len());
        let card = self.cards[index].to_owned();
        self.cards.remove(index);
        card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_new() {
        let deck =  Deck::new();
        println!("{:?}", deck);
        assert_eq!(deck.cards.len(), 22);
    }

    #[test]
    fn test_deck_take_card() {
        let mut deck =  Deck::new();
        let single_card = deck.take_card();
        println!("{:?}", single_card);
        println!("{:?}", deck);
        assert_eq!(deck.cards.len(), 21);
    }
}