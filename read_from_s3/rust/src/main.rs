
mod download;

use std::time::Instant;

fn main() {
    let bucket = "idr";
    let name = "6001247.zarr";
    let prefix = "zarr/v0.1/";
    
    let start = Instant::now();
    download::download_data_default(bucket, prefix, name);
    let duration = start.elapsed();
    println!("Time elapsed in download_data_default() is: {:?}", duration);
}
