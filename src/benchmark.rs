use crate::{
    cli::Cli,
    stats::{Sample, Summary},
};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, Method, Request};
use std::{
    error::Error,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};
use tokio::task::JoinHandle;


pub async fn run(cli: &Cli) -> Result<(Summary, Vec<Sample>), Box<dyn Error>> {

    let method: Method = cli.method.parse()?;
    let client = Client::builder().timeout(cli.timeout).build()?;

    let mut rb = client.request(method, &cli.url);
    for (k, v) in &cli.headers {
        rb = rb.header(k, v);
    }
    if let Some(p) = &cli.body_file {
        rb = rb.body(tokio::fs::read(p).await?);
    }
    let req_tpl: Request = rb.build()?;
    let req_tpl = Arc::new(req_tpl);


    let max_requests = cli.requests.map(|n| n.get() as u64);
    let deadline: Option<Instant> = cli.duration.map(|d| Instant::now() + d);


    let total_sent = Arc::new(AtomicU64::new(0));
    let samples:    Arc<Mutex<Vec<Sample>>> = Arc::new(Mutex::new(Vec::new()));


    let pb = match max_requests {
        Some(n) => ProgressBar::new(n),
        None    => ProgressBar::new_spinner(),
    };
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {elapsed_precise} – sent:{pos} [{eta_precise}]",
        )
        .unwrap(),
    );


    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(cli.concurrency.get());

    for _ in 0..cli.concurrency.get() {
        let client   = client.clone();
        let tpl      = Arc::clone(&req_tpl);
        let samples  = Arc::clone(&samples);
        let sent_ctr = Arc::clone(&total_sent);
        let pb       = pb.clone();
        let max_req  = max_requests;
        let end_by   = deadline;

        handles.push(tokio::spawn(async move {
            loop {
         
                if let Some(cap) = max_req {
                    let prev = sent_ctr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| {
                        if v >= cap { None } else { Some(v + 1) }
                    });
                    if prev.is_err() {
                        break; 
                    }
                } else {
                    sent_ctr.fetch_add(1, Ordering::SeqCst);
                }

             
                if let Some(dl) = end_by {
                    if Instant::now() > dl {
                        break;
                    }
                }

            
                let start = Instant::now();
                let req   = tpl.try_clone().expect("clone request");
                let resp  = client.execute(req).await;
                let dur   = start.elapsed();

                let sample = match resp {
                    Ok(r) => Sample { status: Some(r.status().as_u16()), latency: dur, error: None },
                    Err(e) => Sample { status: None, latency: dur, error: Some(e.to_string()) },
                };

                samples.lock().unwrap().push(sample);
                pb.inc(1);
            }
        }));
    }


    for h in handles { let _ = h.await; }
    pb.finish_and_clear();

    let locked  = samples.lock().unwrap();
    let summary = Summary::from_samples(&locked);
    Ok((summary, locked.clone()))
}
