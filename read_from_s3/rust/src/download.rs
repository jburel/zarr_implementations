use rusoto_signature::region::Region;
use rusoto_core::request::HttpClient;
use rusoto_signature::credential::{AwsCredentials, StaticProvider};
use rusoto_s3::{GetObjectRequest, ListObjectsV2Request, S3Client, S3};

use futures::{Future, Stream};
use std::io::Write;
use std::fs::File;
use std::fs;

pub static ENPOINT: &str = "https://s3.embassy.ebi.ac.uk/";

pub fn download_data(endpoint: &str, bucket: &str, prefix: &str, name: &str) {
    let key = format!("{}{}", prefix, name);
    
    let region = Region::Custom {
                name: "us-west-1".to_owned(),
                endpoint: endpoint.to_owned(),
               };
    let client = S3Client::new_with(
                HttpClient::new().expect("Failed to creat HTTP client"),
                StaticProvider::from(AwsCredentials::default()),
                region);

    let get_req = ListObjectsV2Request {
                  bucket: bucket.to_owned(),
                  prefix: Some(key.to_owned()),
                  ..Default::default()
                };


    println!("{:?}", get_req);

    let result = client.list_objects_v2(get_req).sync().expect("error!");
    let contents_v1 = result.contents.clone().unwrap();
    let v1_iter = contents_v1.iter();
    let directory = format!("{}{}", "./test/", name);
    let dir = fs::create_dir_all(directory.clone());
    println!("{:?}", dir);
    let n = key.len();
    for val in v1_iter {
        let value = val.key.clone().unwrap();
        let m = value.len();
        let substring = &value[(n+1)..m];
        let v;
        if substring.contains(std::path::MAIN_SEPARATOR) {
          let split = substring.split(std::path::MAIN_SEPARATOR);
          let vec: Vec<&str> = split.collect();
          let size = vec.len()-1;
          let mut path = directory.clone();
          for i in 0..size {
              let keydir = format!("{}{}{}", path.clone(), std::path::MAIN_SEPARATOR, vec[i]);
              let _dir = fs::create_dir_all(keydir.clone());
              path = format!("{}{}{}", path.clone(), std::path::MAIN_SEPARATOR, vec[i]);
          }
          v = format!("{}{}{}", path.clone(), std::path::MAIN_SEPARATOR, vec[size]);
          println!("{:?}",  v);
        } else {
          v = format!("{}{}{}", directory.clone(), std::path::MAIN_SEPARATOR, substring);
        }

        
        let req = GetObjectRequest {
                  bucket: bucket.to_owned(),
                  key: value.to_owned(),
                  ..Default::default()};
        let r = client.get_object(req).sync().expect("error!");
        let stream = r.body.unwrap();
        let body = stream.concat2().wait().unwrap();
        let mut file = File::create(v).expect("create failed");
        file.write_all(&body).expect("failed to write body");
    }
}

pub fn download_data_default(bucket: &str, prefix: &str, name: &str) {
    download_data(ENPOINT, bucket, prefix, name)
}
