fn main() {
    // 在这里添加任何自定义构建步骤
    println!("cargo::rerun-if-changed=src");
    // 获取目标平台信息
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();


    // print the platform info
    println!("cargo::warning=target_os: {}", target_os);
    println!("cargo::warning=target_arch: {}", target_arch);

    // 设置库文件路径
    // let lib_path = match (target_os.as_str(), target_arch.as_str()) {
    //     ("macos", _) => "deps/darwin",
    //     ("ios", "aarch64") => "deps/ios",
    //     ("ios", "aarch64") => "deps/ios_sim",  // iOS 模拟器
    //     _ => panic!("Unsupported platform: {} {}", target_os, target_arch),
    // };

    //println!("cargo::rustc-env=IPHONEOS_DEPLOYMENT_TARGET=13.0");

    // 设置链接路径和库
    let current_dir = std::env::current_dir().unwrap();
    println!("cargo::warning=current_dir: {}", current_dir.display());
    //println!("cargo::rustc-link-search={}", current_dir.join(lib_path).display());
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=CoreML");
    // println!("cargo:rustc-link-arg=-framework");
    // println!("cargo:rustc-link-arg=CoreFoundation");
    // println!("cargo:rustc-link-arg=-framework");
    // println!("cargo:rustc-link-arg=Metal");
    // println!("cargo:rustc-link-arg=-framework");
    // println!("cargo:rustc-link-arg=Accelerate");
    println!("cargo::rustc-link-lib=static=onnxruntime");
}
