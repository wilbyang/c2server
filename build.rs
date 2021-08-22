fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["implant.proto"], &["."])
        .unwrap()
}