


// We can include generated rust code in-app using `tonic`.
// Let’s create an `hello.rs` file to reuse in both server and client.

    // this would include code generated for package hello from .proto file
    tonic::include_proto!("hello");
