fn main() {
    tonic_build::configure()
    .out_dir("D:/Vscode/practice-rust/practice-rust/pow/src/pb")
    .compile(&["abi.proto"],&["."])
    .unwrap()
}