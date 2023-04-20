fn main() {
    println!("cargo:rerun-if-changed=.env");
    dotenv_build::output(dotenv_build::Config {
        filename: std::path::Path::new(".env"),
        recursive_search: false,
        fail_if_missing_dotenv: false,
    })
    .unwrap()
}
