



fn main () -> Result<(), Box<dyn std::error::Error>> {

    println!("Compilando proto com build.rs");
    tonic_build::compile_protos("protos/hello.proto")?;
    Ok(())
}
