// trait bound syntax on methods
use std::{collections::HashMap, fmt::{format, Display}};
use std::ops::Add;

pub trait Accommodation { // the visibility of the trait functions are defined by the visibility of the trait
      
    fn book(&mut self, name: &str,nights: u32) -> ();

    // default implementation 
    fn default() {
        println!("This is a default Implementation");
    }
}



pub trait Description {
    fn get_description(&self) -> String {
        "The best Hotel ever".to_string()
    }
}


#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String,u32>
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self { name, reservations: HashMap::new() }
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str,nights: u32) -> () {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name,self.get_description())
    }
}


#[derive(Debug)]
pub struct AirBnB {
    host: String,
    guests: Vec<(String,u32)>
}

impl AirBnB {
    pub fn new(host: &str) -> Self{
        Self { host: host.to_string(), guests: vec!() }
    } 
}


impl Accommodation for AirBnB {

    fn book(&mut self, name: &str,nights: u32) -> () {
        self.guests.push((name.to_string(),nights));
    }
}


impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("The best host in the world {}",self.host)
    }
}


