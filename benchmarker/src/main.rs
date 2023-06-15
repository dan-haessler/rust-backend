use tokio;
use reqwest::Client;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let concurrent_requests = 1000;
  let total_requests = 1000000;
  let client = Client::new();
  let start_time = Instant::now();

  // Send concurrent requests
  let mut tasks = Vec::new();
  for _ in 0..concurrent_requests {
    let client = client.clone();
    let task = tokio::spawn(async move {
      for _ in 0..total_requests / concurrent_requests {
        let _response = client.get("http://localhost:8080/authors").send().await?;
      }
      Ok::<(), reqwest::Error>(())
    });
    tasks.push(task);
  }

  // Wait for all tasks to complete
  for task in tasks {
    let _ = task.await?;
  }

  let elapsed_time = start_time.elapsed();

  println!("Concurrent requests: {}", concurrent_requests);
  println!("Total requests: {}", total_requests);
  println!("Total time: {:?}", elapsed_time);
  println!(
    "Requests per second: {:.2}",
    total_requests as f64 / elapsed_time.as_secs_f64()
  );

  Ok(())
}
