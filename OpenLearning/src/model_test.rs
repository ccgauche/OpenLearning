
use console::style;
use std::fs;

use std::time::Instant;

pub fn test(file: String,sample: String) {
    println!();
    let now = Instant::now();

    let file_string = match fs::read_to_string(&file) {
        Ok(e) => {
            e
        },
        _ => {
            println!("{} {}",style("Unable to open file: ").red().bold(),style(&file).bold());
            return;
        }
    };
    super::terminal_render::set_loading(1,50.0,None);

    for i in file_string.replace("[\"","").replace("\"]","").replace("\", \"","\",\"").split("\",\"") {
        if sample.contains(i) {
            super::terminal_render::set_end(1,now.elapsed().as_millis());
            println!("{} {}",style("Correspond").cyan().bold(),style("true").green().bold());
            return;
        }
    }
    super::terminal_render::set_end(1,now.elapsed().as_millis());
    println!("{} {}",style("Correspond").cyan().bold(),style("false").red().bold())
}
