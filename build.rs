fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
    slint_build::compile_resource("resources.gresource.xml");
}
