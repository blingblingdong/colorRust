use extendr_api::prelude::*;
use text_colorizer::*;
use std::error::Error;
use::std::fs::File;
use std::io::{BufReader, BufRead};
use std::fs;
use std::process::Command;
use std::io::Read;


#[extendr]
fn rust_find(file: &str, type_file: &str) -> Vec<String> {
    // 创建并配置命令
    let output = Command::new("find")
        .arg(file)
        .arg("-type")
        .arg("f")
        .output()
        .expect("failed to execute process");

    let mut file_vec = Vec::new();
    let file_str = String::from_utf8_lossy(&output.stdout);
    for file in file_str.lines() {
        if type_file.is_empty() {
            file_vec.push(file.to_string());
        } else if file.ends_with(type_file) {
            file_vec.push(file.to_string());
        }
    };

    file_vec

}



/// rust_head：用Rust實踐head命令
///
/// 檢視檔案前幾行、前幾位元，也可以檢視多個文件
///
/// @param file 可以是某個根目錄或指定文件
/// @param type_file 文件類型，如txt,csv...，預設是不指定
/// @param n 印出幾行內容，預設10
/// @param byte 印出幾個字元，預設是0(0是全部的意思)
///
/// @examples
/// rust_head("colorRust-Ex.R", "", 10, 0)
///@export
#[extendr]
fn rust_head(file: &str, type_file: &str, n: u32, byte: u32) {
    let vec = rust_find(file, type_file);
    for file_name in vec {
      match open(&file_name){
        Err(err) => println!("{}: {}", file_name, err),
        Ok(mut file) => {
          if byte > 0 {
              rprintln!("==>{}", file_name.green());
              rprintln!("----------------");
              let mut handle = file.take(byte as u64);
              let mut buffer = vec![0; byte as usize];
              let bytes_read = handle.read(&mut buffer).unwrap();
              rprintln!("{}", String::from_utf8_lossy(&buffer[..bytes_read]) );
              rprintln!("----------------");
            } else {
              rprintln!("==>{}", file_name.green());
              rprintln!("----------------");
              let mut line = String::new();
              for _ in 0..n {
                let b = file.read_line(&mut line);
                rprint!("{}", line);
                line.clear();
              }
              rprintln!("----------------");
            }
        }
      }
    }
}


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
    fn rust_head;
}
