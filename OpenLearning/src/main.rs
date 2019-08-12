extern crate rayon;
extern crate console;
#[macro_use]
extern crate lazy_static;
extern crate aho_corasick;
extern crate clap;

mod linear_learning;
mod terminal_render;
mod file_manager;
mod model_test;

use clap::{Arg, App, SubCommand};


#[warn(unused_must_use)]
fn main() {
    let matches = App::new("Open Learning")
                          .version("0.1")
                          .author("ccgauche")
                          .about("Open Learning Client")
                         .subcommand(SubCommand::with_name("learn")
                                     .about("Enable linear learning in OpenLearning")
                                     .version("0.1")
                                     .author("ccgauche | ExpHP")
                                     .arg(Arg::with_name("dataset")
                                         .short("d")
                                         .long("dataset")
                                         .value_name("FILE")
                                         .takes_value(true)
                                         .help("Set the dataset FILE input"))
                                     .arg(Arg::with_name("normal")
                                         .short("n")
                                         .long("normal")
                                         .value_name("FILE")
                                         .takes_value(true)
                                         .help("Set the normal dataset FILE input"))
                                     .arg(Arg::with_name("output")
                                         .short("o")
                                         .long("output")
                                         .value_name("FILE")
                                         .takes_value(true)
                                         .help("Set the FILE model output")))
                          .subcommand(SubCommand::with_name("test")
                                      .about("Test a value with a model")
                                      .version("0.1")
                                      .author("ccgauche")
                                      .arg(Arg::with_name("input")
                                          .short("i")
                                          .long("input")
                                          .value_name("FILE")
                                          .takes_value(true)
                                          .help("Set the model FILE input"))
                                      .arg(Arg::with_name("input_string")
                                          .required(true)
                                          .index(1)
                                          .help("Set the FILE model output")))
                          .get_matches();
    if let Some(re) = matches.subcommand_matches("learn") {
        let dataset = re.value_of("dataset").unwrap_or("dataset.txt").to_string();
        let normal = re.value_of("normal").unwrap_or("normal.txt").to_string();
        let output = re.value_of("output").unwrap_or("model.txt").to_string();
        linear_learning::run_learning(dataset,normal,output);
    } else if let Some(re) = matches.subcommand_matches("test") {
        let model = re.value_of("input").unwrap_or("model.txt").to_string();
        let input = re.value_of("input_string").unwrap();
        model_test::test(model,input.to_string());
    }
}
