fn main() {
    glib_build_tools::compile_resources(
        &["resources/"],
        "resources/resource.xml",
        "resources",
    );
}
