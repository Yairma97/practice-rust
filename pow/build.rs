fn main(){
    tonic_build::configure()
    .out_dir("/Users/mayifan/vscode/practice-rust/pow/src/pb")
    .compile(&["abi.proto"],&["."])
    .unwrap()
}