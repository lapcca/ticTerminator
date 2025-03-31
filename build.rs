fn main(){
    slint_build::compile("ui/open.slint").unwrap();
    slint_build::compile("ui/main.slint").unwrap();
}