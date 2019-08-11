//! # Open Learning API
//!
//! Open Learning API provides a model running client for Open Learning in Rust.
//!
//! ## What is a Open Learning Model?
//!
//! An open learning a model is a system use to find similariries between strings by dataset analysis
//! When the model as been learned no configuration or performance is needed.
//!
//! Example:
//! ```rust
//! extern create open_learning_api;
//!
//! use open_learning_api::*;
//!
//! fn main() {
//!     let model = load_model_from_string("[\" cake\",\"apple pie\"]");
//!     println!("{}",model.match_string("pineapple pen")); // False
//!     println!("{}",model.match_string("cheese cake")); // True
//!
//!     let model = load_model_from_file("model.txt");
//!     println!("{}",model.match_string("pineapple pen")); // False
//!     println!("{}",model.match_string("cheese cake")); // True
//! }
//!
//! ```
//! ## How to generate an Open Learning Model?
//!
//! To create a Open Learning Model please follow the github page of Open Learning:
//!     https://github.com/ccgauche/OpenLearning
//!
//! ## Other libraries:
//!     Open Learning API for JS (OpenLearning4JS)
//!     Open Learning API for Java (OpenLearning4Java)
//!     Open Learning API for Rust - You are here
//!     Open Learning API for Python (OpenLearning4Python)
//!     Open Learning API for PhP (OpenLearning4PhP)
//!
use std::time::{Instant,Duration};
use std::fs::File;
use std::fs;

#[cfg(test)]
mod tests {
    #[test]
    fn match_string() {
        let model = crate::load_model_from_file("model.txt").unwrap();
        assert_eq!(model.match_string("pineapple pen"), false);
        assert_eq!(model.match_string("cheese cake"), true);
    }
    #[test]
    fn load_string() {
        let model = crate::load_model_from_string("[\" cake\",\"apple pie\"]");
        assert_eq!(model.match_string("pineapple pen"), false);
        assert_eq!(model.match_string("cheese cake".to_string()), true);
    }
    #[test]
    fn load_vec() {
        let model = crate::load_model_from_vector(vec![" cake".to_string(),"apple pie".to_string()]);
        assert_eq!(model.match_string("pineapple pen"), false);
        assert_eq!(model.match_string("cheese cake".to_string()), true);
    }
}

pub struct TrainedModel {
    model: Vec<String>,
    reading_time: Duration
}

pub trait ModelMatch {

    fn as_model_string(&self) -> String;
}

impl ModelMatch for String {
    fn as_model_string(&self) -> String {
        self.to_string()
    }
}

impl ModelMatch for &str {
    fn as_model_string(&self) -> String {
        self.to_string()
    }
}

impl TrainedModel {
    pub fn new(model: Vec<String>,reading_time: Duration) -> Self {
        Self {model,reading_time}
    }

    pub fn match_string<E>(&self, s: E) -> bool where E: ModelMatch {
        let u = s.as_model_string();
        for j in &self.model {
            if u.contains(j) {
                return true;
            }
        }
        false
    }
}

pub fn load_model_from_file(file_path: &str) -> Option<TrainedModel> {
    match fs::read_to_string(file_path) {
        Ok(e) => {
            let now = Instant::now();
            let mut vec = Vec::new();
            for i in e.replace("[\"","").replace("\"]","").replace("\", \"","\",\"").split("\",\"") {
                vec.push(i.to_string());
            }
            Some(TrainedModel::new(vec,now.elapsed()))
        },
        _ => {
            None
        }
    }
}

pub fn load_model_from_string(string: &str) -> TrainedModel {
    let now = Instant::now();
    let mut vec = Vec::new();
    for i in string.replace("[\"","").replace("\"]","").replace("\", \"","\",\"").split("\",\"") {
        vec.push(i.to_string());
    }
    TrainedModel::new(vec,now.elapsed())
}

pub fn load_model_from_vector(vec: Vec<String>) -> TrainedModel {
    TrainedModel::new(vec,Instant::now().elapsed())
}
