rustup target add aarch64-apple-ios x86_64-apple-ios
cargo build --target aarch64-apple-ios --release
cargo build --target x86_64-apple-ios --release


# 合并不同架构的静态库
lipo -create -output libyourlib.a target/aarch64-apple-ios/release/libyourlib.a target/x86_64-apple-ios/release/libyourlib.a

# 创建XCFramework
xcodebuild -create-xcframework -library libyourlib.a -headers path/to/headers -output yourlib.xcframework



import Foundation

class ViewController: UIViewController {
    override func viewDidLoad() {
        super.viewDidLoad()

        let rustWrapper = RustWrapper()
        let sileroInstance = rustWrapper.initSilero()
        let audioData: [Int16] = [/* your audio data */]
        let result = rustWrapper.processAudio(audioData, length: audioData.count)
        print("Result: \(result)")
    }
}