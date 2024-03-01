use extendr_api::prelude::*;
use text_colorizer::*;
use std::error::Error;
use::std::fs::File;
use std::io::{BufReader, BufRead};
use std::fs;

#[extendr]
type MyResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}



fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    let f = File::open(filename)?;
    Ok(Box::new(BufReader::new(f)))
}

/// rust_cast：用Rust實踐cat命令
///
/// 選擇以不同的方式顯示文件的內容
///
/// @param file_name 文件名
/// @param number_lines 是否顯示行號
/// @param number_nonblank_lines 是否顯示非空行號
///
/// @examples
/// rust_cat("colorRust-Ex.R", TRUE, FALSE)
///@export
#[extendr]
pub fn rust_cat(file_name: &str, number_lines: bool, number_nonblank_lines: bool)  {
    let config = Config {
        files: vec![String::from(file_name)],
        number_lines,
        number_nonblank_lines,
    };

    for filename in &config.files {
      match open(&filename){
        Ok(file) => {
          for (line_num, line_result) in file.lines().enumerate(){
            let num = (line_num+1).to_string();
            if config.number_lines {
              rprint!("{}.{}\n" , num.red(), line_result.unwrap());
            } else if config.number_nonblank_lines {
                if let Ok(line) = line_result {
                    if !line.is_empty() {
                        rprint!("{}.{}\n" , num.red(), line);
                    } else {
                        rprintln!("{}", line);
                    }
                } else {
                    rprint!("Error reading line.");
                }
            }else {
              rprintln!("{}", line_result.unwrap());
            }

          }
        },
        Err(e) => {
          rprintln!("{} 無法打開 '{}: {:?}",
                    "錯誤:".red().bold(), filename.green(), e);
        }
      }
    }
}



#[extendr]
fn replace_file(file: &str, replacement: &str) {

  let data = match fs::read_to_string(file) {
   Ok(v) => v,
      Err(e) => {
        rprintln!("{} failed to read from file '{}': {:?}",
                    "Error:".red().bold(),file, e);
        return;
      }
    };

   match fs::write(replacement, &data) {
      Ok(_) => {
        rprintln!("{} successfully written to file '{}'",
              "Success:".green().bold(), replacement);
      },
      Err(e) => {
        rprintln!("{} failed to write to file '{}: {:?}",
              "Error:".red().bold(), replacement, e);
        return;
      }
    };
}


#[extendr]
fn colorize_text(text: &str, color: &str) -> String {
    let colored_text = match color {
        "red" => text.red(),
        "green" => text.green(),
        "blue" => text.blue(),
        "yellow" => text.yellow(),
        "cyan" => text.cyan(),
        "magenta" => text.magenta(),
        "black" => text.black(),
        "white" => text.white(),
        _ => text.normal(),
    };

    rprintln!("Colored text: {}", colored_text.to_string());

    colored_text.to_string()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod colorRust;
    fn rust_cat;
}
