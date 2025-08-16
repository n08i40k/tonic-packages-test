pub mod one {
    tonic::include_proto!("shared.unique1.unique1");
}

pub mod two {
    tonic::include_proto!("shared.unique2.unique2");
}

fn main() {
    println!("Hello, world!");
}
