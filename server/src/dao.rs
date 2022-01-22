use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::S3Error;

pub async fn store_challenge(challenge: &str) {
  let bucket_name = "terrylockett-sso";
  let region = "us-east-1".parse().unwrap();
  let credentials = Credentials::default().unwrap();

  let bucket = Bucket::new(bucket_name, region, credentials).unwrap();

  let content = "".as_bytes();
  let result = bucket.put_object(format!("{}", challenge), content).await;
  println!("{:?}", result)
}