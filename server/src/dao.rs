use s3::bucket::Bucket;
use s3::creds::Credentials;

// TODO clean up methods in here so not so much code duplication

static BUCKET_NAME: &str = &"terrylockett-sso";

pub async fn store_challenge(challenge: &str) {
  let region = "us-east-1".parse().unwrap();
  let credentials = Credentials::default().unwrap();
  let bucket = Bucket::new(BUCKET_NAME, region, credentials).unwrap();

  let content = "".as_bytes();
  let result = bucket.put_object(format!("{}", challenge), content).await;
  println!("{:?}", result)
}

pub async fn challenge_exists(challenge: &str) -> bool {
  let region = "us-east-1".parse().unwrap();
  let credentials = Credentials::default().unwrap();
  let bucket = Bucket::new(BUCKET_NAME, region, credentials).unwrap();

  let result = bucket.get_object(format!("{}", challenge)).await;
  match result {
    Ok((_, 200)) => { true },
    Ok((_, 404)) => { false },
    Ok((_, code)) => {
      println!("weird code {}", code);
      false
    }
    Err(e) => {
      println!("error {}", e);
      false
    }
  }
}

pub async fn store_code(code: &str, challenge: &str) {
  let region = "us-east-1".parse().unwrap();
  let credentials = Credentials::default().unwrap();
  let bucket = Bucket::new(BUCKET_NAME, region, credentials).unwrap();

  let content = format!("{}", challenge).clone();
  let result = bucket.put_object(format!("{}", code), content.as_bytes()).await;
  println!("{:?}", result)
}

pub async fn code_exists(code: &str) -> bool{
  let region = "us-east-1".parse().unwrap();
  let credentials = Credentials::default().unwrap();
  let bucket = Bucket::new(BUCKET_NAME, region, credentials).unwrap();

  let result = bucket.get_object(format!("{}", code)).await;
  match result {
    Ok((_, 200)) => { true },
    Ok((_, 404)) => { false },
    Ok((_, code)) => {
      println!("weird code {}", code);
      false
    }
    Err(e) => {
      println!("error {}", e);
      false
    }
  }
}