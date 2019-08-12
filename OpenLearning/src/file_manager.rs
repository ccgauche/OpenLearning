
use console::style;
use std::fs;

use std::time::{Instant,Duration};

use std::fs::File;
use std::io::Write;

use super::terminal_render;

pub struct InputData {
    pub dataset_string: String,
    pub dataset_vector: Vec<String>,
    pub normal_string: String,
    pub normal_vector: Vec<String>,
    pub time: Duration
}

impl InputData {
    pub fn read(dataset_in: String,normal_in: String) -> Option<Self> {
        let now = Instant::now();

        let mut normal: Vec<String> = Vec::new();
        let mut dataset: Vec<String> = Vec::new();

        let dataset_string = match fs::read_to_string(&dataset_in) {
            Ok(e) => {
                e
            },
            _ => {
                println!("{} {}",style("Unable to open file: ").red().bold(),style(&dataset_in).bold());
                return None;
            }
        };
        let normal_string = match fs::read_to_string(&normal_in) {
            Ok(e) => {
                e
            },
            _ => {
                println!("{} {}",style("Unable to open file: ").red().bold(),style(&normal_in).bold());
                return None;
            }
        };
        super::terminal_render::set_loading(1,0.3,None);

        for x in dataset_string.split('\n') {
            if x.len() >= 3 {
                dataset.push(clear(x.trim().to_string()));
            }
        }
        for x in normal_string.split('\n') {
            if x.len() >= 3 {
                normal.push(clear(x.trim().to_string()));
            }
        }
        super::terminal_render::set_end(1,now.elapsed().as_millis());
        Some(Self{
            dataset_string,
            dataset_vector: dataset,
            normal_string,
            normal_vector: normal,
            time: now.elapsed()
        })
    }
}

#[inline(never)]
fn clear(e: String) -> String {
    let mut i = String::new();
    let p = "abcdefghijklmnopqrstuvwxyz0123456789. ";
    for n in e.to_lowercase().split("") {
        if p.contains(n) {
            i.push_str(n);
        }
    }
    i.push_str(" ");
    i
}

pub fn write_data(v: Vec<String>,now: Instant,output_path: String) {
    let o = format!("{:?}",v);
    terminal_render::set_loading(2,50.0,None);
    let mut file = File::create(&output_path).unwrap();
    file.write_all(o.as_bytes()).unwrap();
    terminal_render::set_end(2,now.elapsed().as_millis());
}
