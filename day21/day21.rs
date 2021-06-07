// #[macro_use]
extern crate anyhow;
extern crate itertools;
//use itertools::Itertools;
extern crate regex;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct Product<'a> {
  ingredients: Vec<&'a str>,
  allergens: Vec<&'a str>,
}
impl<'a> Product<'a> {
  fn from_lines(lines: &'a Vec<&str>) -> Vec<Self> {
    let mut v = Vec::new();
    let re = Regex::new(r"([a-z ]+) \(contains ([a-z, ]+)\)").unwrap();
    for l in lines {
      //println!("{}", l);
      let caps = re.captures(l).expect("malformed line");
      let ingredients_str = caps.get(1).expect("missing ingredients").as_str();
      let ingredients_it = ingredients_str.split_whitespace().map(|s| s.trim());
      let allergens_str = caps.get(2).expect("missing allergens").as_str();
      let allergens_it = allergens_str.split_terminator(",").map(|s| s.trim());

      v.push(Product {
        ingredients: ingredients_it.collect(),
        allergens: allergens_it.collect(),
      })
    }
    v
  }
}

#[derive(Clone, Debug)]
struct ProductIngredients<'a>(HashSet<&'a str>);

// Map from an allergen (string) to a list of possible products (list of ingredients (strings)).
#[derive(Clone, Debug)]
struct AllergenCandidates<'a>(HashMap<&'a str, Vec<ProductIngredients<'a>>>);
impl<'a> AllergenCandidates<'a> {
  fn new() -> Self {
    AllergenCandidates(HashMap::new())
  }
}
impl<'a> AllergenCandidates<'a> {
  fn from_products(products: &'a Vec<Product>) -> Self {
    let mut candidates = AllergenCandidates::new();
    for p in products {
      for a in &p.allergens {
        match candidates.0.get_mut(a) {
          Some(vec_product_ingredients) => {
            vec_product_ingredients.push(ProductIngredients(HashSet::from_iter(p.ingredients.iter().map(|&s| s))));
          }
          None => {
            candidates.0.insert(a, vec![ProductIngredients(HashSet::from_iter(p.ingredients.iter().map(|&s| s)))]);
          }
        }
      }
    }
    candidates
  }
 
  // Walks through the candidates. If an ingredient in a ProductIngredients
  // list is not present in every ProductIngredients, it's removed as
  // candidate from all ProductIngredients.
  fn remove_unmatched(&mut self) {
    for (_allergen, allergen_products) in &mut self.0 {
      // This is the number of products that list the allergen. Any ingredient that
      // could contain the allergen would have to appear in every such product.
      let num_products = allergen_products.len();

      // Count how many times each ingredient appears.
      let mut ingredients_count: HashMap<&str, usize> = HashMap::new();
      for product_ingredients in allergen_products.iter() {
        for &ingredient in &product_ingredients.0 {
          *ingredients_count.entry(ingredient).or_insert(0) += 1;
        }
      }

      let unmatched_ingredients: HashSet<&str> =
        ingredients_count.iter().filter_map(|(&ingredient, &count)| {
          assert!(count <= num_products);
          if count == num_products {
            None  // Appeared the right number of times.
          } else {
            Some(ingredient)  // Not enough, unmatched.
          }
        }).collect();
      // Remove ingredients that can't match the allergen from all products.
      for product_ingredients in allergen_products.iter_mut() {
        product_ingredients.0 = product_ingredients.0.drain().filter(|ingredient| {
          !unmatched_ingredients.contains(ingredient)
        }).collect();
      }
    }
  }
}

#[derive(Clone, Debug)]
struct AllergenSearch<'a>(HashMap<&'a str, ProductIngredients<'a>>);
impl<'a> AllergenSearch<'a> {
  fn from_candidates(candidates: AllergenCandidates<'a>) -> Self {
    let mut map = HashMap::new();
    for (allergen, vec_products) in candidates.0.into_iter() {
      map.insert(allergen, vec_products[0].clone());
    }
    AllergenSearch(map)
  }

  fn reduce_and_find(&mut self) -> AllergenMatch {
    let mut known = AllergenMatch::new();
    loop {
      // Find a candidate with a single ingredient.
      let mut found = None;
      for (allergen, ingredients) in self.0.iter() {
        if known.from_allergen.contains_key(allergen) { continue }
        if ingredients.0.len() == 1 {
          let ingredient = *ingredients.0.iter().next().unwrap();
          known.from_allergen.insert(allergen, ingredient);
          known.from_ingredient.insert(ingredient, allergen);
          found = Some((allergen.to_owned(), ingredient.to_owned()));
          break;
        }
      }
  
      if let Some((found_allergen, found_ingredient)) = found {
        // Remove the found allergen as a candidate in all other 
        for (other_allergen, other_product) in self.0.iter_mut() {
          if other_allergen != &found_allergen {
            other_product.0.remove(&*found_ingredient);
          }
        }
      } else {
        // Hopefully everything is known now...
        break;
      }
    }
    known
  }
}

// A map from ingredient to allergen name.
#[derive(Clone, Debug)]
struct AllergenMatch<'a> {
  from_allergen: HashMap<&'a str, &'a str>,
  from_ingredient: HashMap<&'a str, &'a str>,
}
impl<'a> AllergenMatch<'a> {
  fn new() -> Self {
    AllergenMatch{from_allergen: HashMap::new(), from_ingredient: HashMap::new()}
  }
}

fn solve(input_all: String) -> anyhow::Result<()> {
  let lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  let products: Vec<Product> = Product::from_lines(&lines);

  let mut candidates = AllergenCandidates::from_products(&products);
  candidates.remove_unmatched();
  let mut search = AllergenSearch::from_candidates(candidates);
  let known = search.reduce_and_find();

  let mut count_no_allergen_ingredients = 0;
  for p in &products {
    for i in &p.ingredients {
      if !known.from_ingredient.contains_key(i) {
        count_no_allergen_ingredients += 1;
      }
    }
  }

  println!("Part 1 {}", count_no_allergen_ingredients);

  let mut known_vec: Vec<(&str, &str)> = known.from_allergen.into_iter().collect();
  known_vec.sort_by(|(allergen1, _), (allergen2, _)| { allergen1.partial_cmp(allergen2).unwrap() });
  let mut s = String::new();
  for (_allergen, ingredient) in known_vec {
    if !s.is_empty() {
      s.push(',')
    }
    s.push_str(ingredient);
  }
  println!("Part 2 {}", s);

  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = std::fs::read_to_string("day21/input.txt")?;
  //let input_all = std::fs::read_to_string("day21/test.txt")?;
  solve(input_all)?;
  Ok(())
}