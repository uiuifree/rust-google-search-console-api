use std::error::Error;
use csv::Writer;
use yup_oauth2::{AccessToken};
use google_analytics_api_ga4::search_analytics::query::SearchAnalyticsQueryRequest;
use google_analytics_api_ga4::SearchConsoleApi;
use google_analytics_api_ga4::types::DIMENSION;


async fn test_token() -> AccessToken {
    // 認証
    let secret = yup_oauth2::read_service_account_key("./test.json")
        .await
        .expect("test.json");
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(secret).build().await.unwrap();
    let scopes = &["https://www.googleapis.com/auth/webmasters.readonly"];

    let token = auth.token(scopes).await;
    assert!(token.is_ok(), "{}", token.err().unwrap().to_string());
    token.unwrap()
}


#[tokio::test]
async fn test_run_realtime_report() {
    export("2022-01-01", "2022-01-30").await;
}

async fn export(start_date: &str, end_date: &str) {
    let token = test_token().await;
    let res = SearchConsoleApi::search_analytics().query(token.as_str(), "https://example.com", SearchAnalyticsQueryRequest {
        start_date: start_date.to_string(),
        end_date: end_date.to_string(),
        dimensions: Some(vec![
            DIMENSION::Page
        ]),
        search_type: None,
        query_type: None,
        dimension_filter_groups: None,
        aggregation_type: None,
        row_limit: Some(25000),
        start_row: None,
        data_state: None,
    }).await;
    assert!(res.is_ok(), "{:?}", res.err().unwrap().to_string());
    let data = res.unwrap();
    let mut  inserts = vec![];
    let headers = vec![
        "start".to_string(),
        "end".to_string(),
        "key_page".to_string(),
        "clicks".to_string(),
        "impressions".to_string(),
        "ctr".to_string(),
        "position".to_string(),
    ];
    inserts.push(headers);
    for row in data.rows.unwrap_or_default() {
        let mut insert = vec![];
        insert.push(start_date.to_string());
        insert.push(end_date.to_string());
        for key in row.keys.unwrap_or_default(){
            insert.push(key.to_string());
        }
        insert.push(row.clicks.unwrap_or_default().to_string());
        insert.push(row.impressions.unwrap_or_default().to_string());
        insert.push(row.ctr.unwrap_or_default().to_string());
        insert.push(row.position.unwrap_or_default().to_string());
        inserts.push(insert);
    }
    let _ = example(format!("./sc_{}.csv",start_date).as_str(),inserts);
}

fn example(path: &str, rows: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    for row in rows {
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}