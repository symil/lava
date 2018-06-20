extern crate gcc;

fn main() {
    gcc::Build::new()
               .file("c/bar.c")
               .compile("libbar.a");
}