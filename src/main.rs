use traitsProject::lodging::{Accommodation, Hotel, AirBnB,Description};
use traitsProject::utils;

fn main() {
 
    let h1 = Hotel::new("jooo".to_string());

    println!("{}",h1.summarize());

    let h2 = Hotel::new(vec!["aa".to_string()]);
    println!("{}",h2.get_description());
    
    let mut hotel = Hotel::new(String::from("The Luxe"));
    let mut airbnb = AirBnB::new("Peter");

    let stays: Vec<&dyn Description> = vec![&hotel,&airbnb];

    println!("blahhhh");
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());

    hotel.book("nnnnn", 5); // can be accesed becuase trait is public

    Hotel::<String>::default();

    utils::mix_and_match(&mut hotel, &mut airbnb, "gelloo");


}




// A trait object is an instance of a type that implements a particular trait whose methods will be accessed at runtime using a feature called dynamic dispatch

// Trait must be in scope to Use its Definitions


// traitsProject is the name of crate
// the visibility of the trait functions are defined by the visibility of the trait