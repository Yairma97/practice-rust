fn main() {
    prost_build::Config::new()
        .out_dir("D:/Vscode/practice-rust/practice-rust/imagespec/src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}