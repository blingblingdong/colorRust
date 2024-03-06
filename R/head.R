#' R_head：用Rust實踐head命令
#'
#' 檢視檔案前幾行、前幾位元，也可以檢視多個文件
#'
#' @param dir 可以是某個根目錄或指定文件
#' @param file_type 文件類型，如txt,csv...，預設是不指定
#' @param line 印出幾行內容，預設10
#' @param byte 印出幾個字元，預設是0，0是全部的意思
#'
#' @examples
#' R_head("colorRust-Ex.R", "", 10, 0)
#' @export
R_head <- function(dir, file_type = "", line = 10, byte = 0){
  rust_head(dir, file_type, line, byte)
}
