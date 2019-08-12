use rayon::prelude::*;
use std::sync::Mutex;

use std::time::Instant;
use std::collections::BTreeMap;

use aho_corasick::AhoCorasick;
use itertools::Itertools;

use super::terminal_render;

use super::file_manager::{InputData, write_data};


/*
#[warn(unused_must_use)]
fn main() {
    if env::args().len() == 2 {
        let now1 = Instant::now();
        match fs::read_to_string("model.txt") {
            Ok(e) => {
                let mut i = true;
                let mut w = String::new();
                for argument in env::args() {
                    if i {
                        i = false;
                        continue;
                    }
                    w = argument.to_string();
                }
                println!("  {} {}",style("Intialized in").green().bold(),style(format!("{}ms",now1.elapsed().as_nanos() as f32 / 1_000_000.0)).yellow().bold());
                let now1 = Instant::now();
                w = clear(w);
                for i in e.replace("[\"","").replace("\"]","").replace("\", \"","\",\"").split("\",\"") {
                    if w.contains(i) {
                        println!("  {} {}",style("Search done in").green().bold(),style(format!("{}ms",now1.elapsed().as_nanos() as f32 / 1_000_000.0)).yellow().bold());
                        println!("  {} {}",style("Correspond").yellow().bold(),style("True").green().bold());
                        return;
                    }
                }
                println!("  {} {}",style("Search done in").green().bold(),style(format!("{}ms",now1.elapsed().as_nanos() as f32 / 1_000_000.0)).yellow().bold());
                println!("  {} {}",style("Correspond").yellow().bold(),style("False").red().bold());
            },
            _ => {
                println!("{} {}",style("Failed to open file").red().bold(),style("MODEL.txt").yellow().bold().italic())
            }
        }
        return;
    }
}*/

pub fn run_learning(dataset_path: String,normal_path: String,output_path: String) {
    println!();

    let data = match InputData::read(dataset_path,normal_path) {
        Some(e) => {
            e
        },
        _ => {
            return;
        }
    };

    let now = Instant::now();
    terminal_render::set_loading(0,0.0,None);
    let finished: Mutex<terminal_render::Type> = Mutex::new(terminal_render::Type::new(0));
    let ln = data.dataset_vector.len() as f32;

    let lastp: Mutex<terminal_render::Type> = Mutex::new(terminal_render::Type::new(0));
    let u: Vec<String> = data.dataset_vector.par_iter().flat_map(|p| {
        let hu = input(p.to_string(),&data.normal_string);
        let dat = filter(&hu,&data.dataset_vector);

        if let Ok(mut e) = finished.lock() {
            e.tp += 1;
            let y = (e.tp  as f32)/ln * 100.0;
            if let Ok(mut i) = lastp.lock() {
                if i.tp < y as u32 {
                    terminal_render::set_loading(0,y,Some(temps_restant(&now,y)));
                    i.tp = y as u32;
                }
            }
        }

        dat
    }).collect();
    terminal_render::set_end(0,now.elapsed().as_millis());

    let now = Instant::now();
    terminal_render::set_loading(2,0.0,None);
    let o = filter(&u,&data.dataset_vector);
    write_data(o,now,output_path);
}

fn temps_restant(i: &Instant,p: f32) -> u32 {
    let ela = (i.elapsed().as_millis()/100) as f32 / 10.0;
    let time_pp = ela/p;
    (time_pp*(100.0-p)) as u32
}

#[inline(never)]
fn input(b: String, normal: &str) -> Vec<String> {
    let mut dat: Vec<String> = Vec::with_capacity(b.len());
    for x in 0..(b.len() - 3) {
      for y in 3..b.len()-x {
          let str = &b[x..(x+y)];
          if normal.contains(str) {
              continue;
            }
          dat.push(str.to_string());
        }
    }
    dat
}

fn filter(dat: &[String], vpn: &[String]) -> Vec<String> {
  filter_lua(filter_gdat(dat, vpn))
}

#[inline(never)]
fn filter_gdat(dat: &[String], vpn: &[String]) -> Vec<String> {
  let ac = AhoCorasick::new(dat);
  let matched_indices = {
    vpn.iter().flat_map(|haystack| {
        ac.find_overlapping_iter(haystack).map(|m| m.pattern())
          .unique() // only count the first match of each needle in the haystack
    })
  };

  get_counts(matched_indices).into_iter()
    .filter(|&(_, count)| count > 1)
    .map(|(index, _)| dat[index].to_string())
    .collect()
}

fn get_counts<T: Ord, I: IntoIterator<Item=T>>(iter: I) -> BTreeMap<T, usize> {
    let mut map = BTreeMap::new();
    for value in iter {
        *map.entry(value).or_insert(0) += 1;
    }
    map
}

#[inline(never)]
fn filter_lua(gdat: Vec<String>) -> Vec<String> {
  let mut lua: Vec<String> = Vec::new();
  'loop3: for x in &gdat {
    for y in &gdat {
      if y.to_string().contains(x) && y.len() > x.len() {
        continue 'loop3;
      }
    }
    if !lua.contains(&x) {
      lua.push(x.to_string());
    }
  }
  lua
}
