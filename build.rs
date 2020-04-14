fn main() {
    // This works
    // LD_LIBRARY_PATH=$(pwd)/so/bin/shared/ cargo run
    // This configuration works only when doing cargo run
    // If you want to run the rust binary it is needed to 
    // create the variable LD_LIBRARY_PATH or the shared object
    // has to be copied to /usr/lib and chmod 775
    //
    println!("cargo:rustc-link-search=/home/algolook/Documents/playground/binding/c_code/so/bin/shared\n\
    cargo:rustc-env=LD_LIBRARY_PATH=/home/algolook/Documents/playground/binding/c_code/so/bin/shared\n\
    cargo:rustc-link-lib=dylib=tq84");
}


