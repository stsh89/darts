use anyhow;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../protobuf/proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["../protobuf/proto/games.proto"], &["../protobuf/proto"])?;

    Ok(())
}
