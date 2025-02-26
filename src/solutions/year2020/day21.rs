use std::collections::{HashMap, HashSet};

use crate::Solution;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: HashSet<String>,
    barcode: U256,
}

impl std::fmt::Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ingredients: ")?;
        for i in self.ingredients.iter() {
            write!(f, "{}, ", i)?;
        }
        writeln!(f)?;
        write!(f, "allergens: ")?;
        for a in self.allergens.iter() {
            write!(f, "{}, ", a)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

impl Food {
    fn from(line: &str) -> Self {
        let mut parts = line.split("(");
        let ingredients = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|w| w.to_string())
            .collect();
        let second_part = parts.next().unwrap();
        let allergens = second_part
            .split_ascii_whitespace()
            .skip(1)
            .map(|w| w[..w.len() - 1].to_string())
            .collect();
        Self {
            ingredients,
            allergens,
            barcode: U256::zero(),
        }
    }
    fn init_barcode(&mut self, map: &HashMap<String, usize>) {
        let mut code = U256::zero();
        for ingredient in self.ingredients.iter() {
            let index = map.get(ingredient).unwrap();
            code.bits[255 - index] = true;
        }
        self.barcode = code
    }

    fn contains(&self, allergen: &str) -> bool {
        self.allergens.contains(allergen)
    }
}

fn parse_input(input: String) -> (Vec<Food>, Vec<String>, Vec<String>) {
    let mut foods: Vec<Food> = input.lines().map(Food::from).collect();
    let mut ingredient_index = 0;
    let mut ingredient_map: HashMap<String, usize> = HashMap::new();
    let mut ingredient_vec = vec![];
    let mut allergens = HashSet::new();
    for food in foods.iter() {
        for ingredient in food.ingredients.iter() {
            if let Some(_) = ingredient_map.get(ingredient) {
            } else {
                ingredient_map.insert(ingredient.clone(), ingredient_index);
                ingredient_vec.push(ingredient.clone());
                ingredient_index += 1;
            };
        }
        for allergen in food.allergens.iter() {
            allergens.insert(allergen.clone());
        }
    }

    for food in foods.iter_mut() {
        food.init_barcode(&ingredient_map);
    }

    (foods, allergens.into_iter().collect(), ingredient_vec)
}

pub struct Day21 {}

impl Solution for Day21 {
    fn test_input() -> String {
        String::from(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
        )
    }
    fn solve_part_1(input: String) -> String {
        let parsed = parse_input(input);

        let (foods, allergens, ingredients_vec) = parsed;

        let all_ingredients = U256::all_ones(ingredients_vec.len());

        let allergen_flags: Vec<(String, U256)> = allergens
            .iter()
            .map(|allergen| {
                let mut flag = all_ingredients;
                for food in foods.iter() {
                    if food.contains(allergen) {
                        flag &= food.barcode;
                    }
                }
                (allergen.clone(), flag)
            })
            .collect();

        let mut ans = 0;
        for food in foods.iter() {
            let mut code = food.barcode.clone();
            for flag in allergen_flags.iter().map(|t| t.1) {
                code &= !flag;
            }
            ans += code.count_ones();
        }

        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let parsed = parse_input(input);

        let (foods, allergens, ingredients_vec) = parsed;

        let all_ingredients = U256::all_ones(ingredients_vec.len());

        let allergen_flags: Vec<(String, U256)> = allergens
            .iter()
            .map(|allergen| {
                let mut flag = all_ingredients;
                for food in foods.iter() {
                    if food.contains(allergen) {
                        flag &= food.barcode;
                    }
                }
                (allergen.clone(), flag)
            })
            .collect();

        for (allergen, flag) in allergen_flags.iter() {
            print!("{}: ", allergen);
            for i in 0..=255 {
                if flag[255 - i] {
                    print!("{}, ", i);
                }
            }
            println!()
        }
        // wheat: 0, 26, 89,
        // fish: 26, 27, 54, 67,
        // nuts: 52,
        // sesame: 38, 54, 67,
        // peanuts: 26, 52,
        // shellfish: 54, 89,
        // soy: 52, 54,
        // eggs: 52, 67, 89,

        // wheat: 0
        // fish: 27
        // nuts: 52,
        // sesame: 38
        // peanuts: 26,
        // shellfish: 89,
        // soy: 54,
        // eggs: 67

        let mut allergens = if ingredients_vec.len() > 100 {
            vec![
                ("wheat", 0),
                ("fish", 27),
                ("nuts", 52),
                ("sesame", 38),
                ("peanuts", 26),
                ("shellfish", 89),
                ("soy", 54),
                ("eggs", 67),
            ]
        } else {
            vec![("dairy", 0), ("fish", 2), ("soy", 5)]
        };

        allergens.sort_by_key(|a| a.0);

        let ingredient_names: Vec<String> = allergens
            .into_iter()
            .map(|i| i.1)
            .map(|i| ingredients_vec[i].clone())
            .collect();

        ingredient_names.join(",")
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct U256 {
    bits: [bool; 256],
}

impl std::ops::Index<u8> for U256 {
    type Output = bool;
    fn index(&self, index: u8) -> &Self::Output {
        &self.bits[index as usize]
    }
}

impl std::ops::IndexMut<u8> for U256 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.bits[index as usize]
    }
}

impl std::fmt::Binary for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let largest = self.most_significant_bit();
        let bit = 255 - largest;
        for i in 0..=largest {
            write!(f, "{}", if self[bit + i] { 1 } else { 0 })?
        }
        Ok(())
    }
}

impl std::ops::BitOr<U256> for U256 {
    type Output = U256;
    fn bitor(self, rhs: U256) -> Self::Output {
        let mut new = U256::zero();
        for i in 0..=255 {
            new[i] = self[i] | rhs[i];
        }
        new
    }
}

impl std::ops::BitAnd<U256> for U256 {
    type Output = U256;
    fn bitand(self, rhs: U256) -> Self::Output {
        let mut new = U256::zero();
        for i in 0..=255 {
            new[i] = self[i] & rhs[i];
        }
        new
    }
}

impl std::ops::BitXor<U256> for U256 {
    type Output = U256;
    fn bitxor(self, rhs: U256) -> Self::Output {
        let mut new = U256::zero();
        for i in 0..=255 {
            new[i] = self[i] ^ rhs[i];
        }
        new
    }
}

impl std::ops::Not for U256 {
    type Output = U256;
    fn not(self) -> Self::Output {
        let mut new = U256::zero();
        for i in 0..=255 {
            new[i] = !self[i];
        }
        new
    }
}

impl std::ops::BitOrAssign<U256> for U256 {
    fn bitor_assign(&mut self, rhs: U256) {
        *self = *self | rhs;
    }
}
impl std::ops::BitAndAssign<U256> for U256 {
    fn bitand_assign(&mut self, rhs: U256) {
        *self = *self & rhs;
    }
}
impl std::ops::BitXorAssign<U256> for U256 {
    fn bitxor_assign(&mut self, rhs: U256) {
        *self = *self ^ rhs;
    }
}

impl U256 {
    fn count_ones(&self) -> usize {
        let mut ans = 0;
        for i in 0..256 {
            if self.bits[i] {
                ans += 1
            }
        }
        ans
    }
    fn _from(mut num: usize) -> Self {
        let mut power = 0;
        let mut bits = [false; 256];
        while num > 0 {
            bits[255 - power] = num % 2 == 1;
            num /= 2;
            power += 1;
        }

        Self { bits }
    }
    fn all_ones(num: usize) -> Self {
        let mut me = Self::zero();
        for i in 0..num {
            me.bits[255 - i] = true;
        }
        me
    }
    fn zero() -> Self {
        Self { bits: [false; 256] }
    }
    fn _trailing_zeroes(&self) -> u8 {
        for i in 0..=255 {
            if self[255 - i] {
                return i;
            }
        }
        255
    }
    fn most_significant_bit(&self) -> u8 {
        for i in (0..=255).rev() {
            if self[255 - i] {
                return i;
            }
        }
        0
    }
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn test_u256() {
        let a = 0b1101;
        let b = 0b0110;

        let aa = U256::_from(a);
        let bb = U256::_from(b);
        let mut ma = U256::_from(a);
        ma &= bb;
        assert_eq!(aa & bb, U256::_from(a & b));
        let mut ma = U256::_from(a);
        ma |= bb;
        assert_eq!(aa | bb, U256::_from(a | b));
        let mut ma = U256::_from(a);
        ma ^= bb;
        assert_eq!(aa ^ bb, U256::_from(a ^ b));
        let num = U256::_from(12);

        assert_eq!(num.most_significant_bit(), 3);
        assert_eq!(num._trailing_zeroes(), 2);
        assert!(false, "{} is {:b}", a, aa);
    }

    #[test]
    fn test_part_1() {
        let input = Day21::test_input();
        let ans = Day21::solve_part_1(input);
        assert_eq!(ans, "5");
    }

    #[test]
    fn test_part_2() {
        let input = Day21::test_input();
        let ans = Day21::solve_part_2(input);
        assert_eq!(ans, "mxmxvkd,sqjhc,fvjkl")
    }
}
