use std::collections::HashMap;

use axum::{routing::*, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
enum Aggregation {
    Sum,
    Avg,
}

type TimeSeriesData = Vec<(i32, f32)>;
type BucketToPrimos = HashMap<String, Vec<i32>>;
type TagsToData = HashMap<i32, TimeSeriesData>;
type BucketsToTags = HashMap<String, TagsToData>;

#[derive(Deserialize)]
struct ChartDataReq {
    primo_id_buckets: BucketToPrimos,
    start_unix_date: i32,
    stop_unix_date: i32,
    tag_ids: Vec<i32>,
    aggregation: Aggregation,
}

#[derive(Serialize)]
struct ChartDataRes {
    data: BucketsToTags,
}

fn get_bucket_data(primos: &Vec<i32>, tags: &Vec<i32>) -> TagsToData {
    let mut tag_buckets = TagsToData::with_capacity(tags.len());
    for &tag in tags {
        tag_buckets.insert(tag, vec![(23, 412.12), (69, 512.23), (420, 2828.23)]);
    }
    tag_buckets
}

async fn get_data(Json(req): Json<ChartDataReq>) -> Json<ChartDataRes> {
    let mut buckets = BucketsToTags::with_capacity(req.primo_id_buckets.len());
    for (bucket_name, primos) in req.primo_id_buckets {
        buckets.insert(bucket_name, get_bucket_data(&primos, &req.tag_ids));
    }
    Json(ChartDataRes { data: buckets })
}

pub fn build_router() -> Router {
    Router::new().route("/", post(get_data))
}
