fn main() {
    let test_foo = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", test_foo);
    println!("cargo:rustc-cfg=feature={}","\"pass\"".to_string());
}