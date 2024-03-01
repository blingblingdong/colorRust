
# Some useful keyboard shortcuts for package authoring:
#
#   Install Package:           'Cmd + Shift + B'
#   Check Package:             'Cmd + Shift + E'
#   Test Package:              'Cmd + Shift + T'


#' R_cat：用Rust實踐cat命令
#'
#' 選擇以不同的方式顯示文件的內容
#'
#' @param file 文件名
#' @param number_lines 是否顯示行號
#' @param number_nonblank_lines 是否顯示非空行號
#' @return: NULL
#' @export
#' @examples
#' R_cat("colorRust-Ex.R", TRUE, FALSE)
R_cat <- function(file, number_lines = FALSE, number_nonblank_lines = FALSE){

  rust_cat(file, number_lines, number_nonblank_lines)

}
