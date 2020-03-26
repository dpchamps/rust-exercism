pub struct Allergies {
    score : u32
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies{
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (Allergies::get_bitfield(allergen) & self.score) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
         ALLERGENS.to_vec().into_iter().filter(|x| self.is_allergic_to(x)).collect()
    }

    fn get_bitfield(allergen : &Allergen) -> u32{
        match allergen {
            Allergen::Eggs => 1 << 0,
            Allergen::Peanuts => 1 << 1,
            Allergen::Shellfish => 1 << 2,
            Allergen::Strawberries => 1 << 3,
            Allergen::Tomatoes => 1 << 4,
            Allergen::Chocolate => 1 << 5,
            Allergen::Pollen => 1 << 6,
            Allergen::Cats => 1 << 7,
        }
    }
}
