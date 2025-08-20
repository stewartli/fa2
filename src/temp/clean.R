library(magrittr)

config <- config::get(
  file = file.path(Sys.getenv("USER_FA_DIR"), "/faproj/box/config.yml")
)

options(box.path = config$rbox)
getOption("box.path")

box::use(stbox / box)
box$hello()
box$check_r_pkg()
box$create_rsproj()
