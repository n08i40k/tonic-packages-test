fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["proto/one.proto", "proto/two.proto"], &["proto"])
        .unwrap();
}
