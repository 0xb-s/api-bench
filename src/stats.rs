use serde::Serialize;
use std::time::Duration;


mod ser {
    use serde::Serializer;
    use std::time::Duration;
    pub fn micros<S>(d: &Duration, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        s.serialize_u64(d.as_micros() as u64)
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct Sample {
    pub status:  Option<u16>,
    #[serde(serialize_with = "ser::micros")]
    pub latency: Duration,
    pub error:   Option<String>,
}


#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub total:   usize,
    pub success: usize,
    pub errors:  usize,
    pub rps:     f64,
    #[serde(serialize_with = "ser::micros")]
    pub mean_latency: Duration,
    #[serde(serialize_with = "ser::micros")]
    pub p50_latency:  Duration,
    #[serde(serialize_with = "ser::micros")]
    pub p95_latency:  Duration,
    #[serde(serialize_with = "ser::micros")]
    pub p99_latency:  Duration,
}

impl Summary {
    pub fn from_samples(samples: &[Sample]) -> Self {
        let mut lat: Vec<u64> = Vec::with_capacity(samples.len());
        let mut ok = 0usize;
        for s in samples {
            if s.error.is_none() {
                ok += 1;
                lat.push(s.latency.as_micros() as u64);
            }
        }
        lat.sort_unstable();
        let pct = |p: f64| -> u64 {
            if lat.is_empty() { 0 } else {
                let idx = ((p / 100.0) * (lat.len() - 1) as f64).round() as usize;
                lat[idx]
            }
        };
        let mean = if lat.is_empty() {
            0
        } else {
            (lat.iter().map(|&v| v as u128).sum::<u128>() / lat.len() as u128) as u64
        };
        let wall_us = lat.iter().copied().max().unwrap_or(1);
        let rps = samples.len() as f64 / (wall_us as f64 / 1_000_000.0);

        Summary {
            total: samples.len(),
            success: ok,
            errors: samples.len() - ok,
            rps,
            mean_latency: Duration::from_micros(mean),
            p50_latency:  Duration::from_micros(pct(50.0)),
            p95_latency:  Duration::from_micros(pct(95.0)),
            p99_latency:  Duration::from_micros(pct(99.0)),
        }
    }
}
