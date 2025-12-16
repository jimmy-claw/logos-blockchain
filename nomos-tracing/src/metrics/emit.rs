use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use opentelemetry::{
    KeyValue, Value, global,
    metrics::{Counter, Gauge, Histogram, Meter},
};

fn meter() -> Meter {
    global::meter("nomos-node")
}

static U64_COUNTERS: OnceLock<Mutex<HashMap<&'static str, Counter<u64>>>> = OnceLock::new();
static F64_COUNTERS: OnceLock<Mutex<HashMap<&'static str, Counter<f64>>>> = OnceLock::new();
static U64_GAUGES: OnceLock<Mutex<HashMap<&'static str, Gauge<u64>>>> = OnceLock::new();
static U64_HISTOGRAMS: OnceLock<Mutex<HashMap<&'static str, Histogram<u64>>>> = OnceLock::new();
static F64_HISTOGRAMS: OnceLock<Mutex<HashMap<&'static str, Histogram<f64>>>> = OnceLock::new();

pub trait IntoMetricValue {
    fn into_metric_value(self) -> Value;
}

impl IntoMetricValue for Value {
    fn into_metric_value(self) -> Value {
        self
    }
}

impl IntoMetricValue for &str {
    fn into_metric_value(self) -> Value {
        Value::from(self.to_owned())
    }
}

impl IntoMetricValue for String {
    fn into_metric_value(self) -> Value {
        Value::from(self)
    }
}

impl IntoMetricValue for u16 {
    fn into_metric_value(self) -> Value {
        Value::from(i64::from(self))
    }
}

pub fn key_value(key: &'static str, value: impl IntoMetricValue) -> KeyValue {
    KeyValue::new(key, value.into_metric_value())
}

pub trait IntoMetricU64 {
    fn into_metric_u64(self) -> u64;
}

impl IntoMetricU64 for u64 {
    fn into_metric_u64(self) -> u64 {
        self
    }
}

impl IntoMetricU64 for usize {
    fn into_metric_u64(self) -> u64 {
        u64::try_from(self).unwrap_or(u64::MAX)
    }
}

impl IntoMetricU64 for u32 {
    fn into_metric_u64(self) -> u64 {
        u64::from(self)
    }
}

impl IntoMetricU64 for i32 {
    fn into_metric_u64(self) -> u64 {
        u64::try_from(self).unwrap_or(0)
    }
}

fn u64_counter(name: &'static str) -> Counter<u64> {
    let mut counters = U64_COUNTERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("u64 counter lock poisoned");

    counters
        .entry(name)
        .or_insert_with(|| meter().u64_counter(name).init())
        .clone()
}

fn f64_counter(name: &'static str) -> Counter<f64> {
    let mut counters = F64_COUNTERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("f64 counter lock poisoned");

    counters
        .entry(name)
        .or_insert_with(|| meter().f64_counter(name).init())
        .clone()
}

fn u64_gauge(name: &'static str) -> Gauge<u64> {
    let mut gauges = U64_GAUGES
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("u64 gauge lock poisoned");

    gauges
        .entry(name)
        .or_insert_with(|| meter().u64_gauge(name).init())
        .clone()
}

fn u64_histogram(name: &'static str) -> Histogram<u64> {
    let mut histograms = U64_HISTOGRAMS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("u64 histogram lock poisoned");

    histograms
        .entry(name)
        .or_insert_with(|| meter().u64_histogram(name).init())
        .clone()
}

fn f64_histogram(name: &'static str) -> Histogram<f64> {
    let mut histograms = F64_HISTOGRAMS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("f64 histogram lock poisoned");

    histograms
        .entry(name)
        .or_insert_with(|| meter().f64_histogram(name).init())
        .clone()
}

pub fn counter_u64(name: &'static str, value: impl IntoMetricU64, attributes: &[KeyValue]) {
    u64_counter(name).add(value.into_metric_u64(), attributes);
}

pub fn counter_f64(name: &'static str, value: f64, attributes: &[KeyValue]) {
    f64_counter(name).add(value, attributes);
}

pub fn gauge_u64(name: &'static str, value: impl IntoMetricU64, attributes: &[KeyValue]) {
    u64_gauge(name).record(value.into_metric_u64(), attributes);
}

pub fn histogram_u64(name: &'static str, value: impl IntoMetricU64, attributes: &[KeyValue]) {
    u64_histogram(name).record(value.into_metric_u64(), attributes);
}

pub fn histogram_f64(name: &'static str, value: f64, attributes: &[KeyValue]) {
    f64_histogram(name).record(value, attributes);
}
