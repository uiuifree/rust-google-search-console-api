use yup_oauth2::{AccessToken};
use google_search_console_api::{MobileFriendlyTestApi, SearchConsoleApi};
use google_search_console_api::mobile_friendly_test::RequestMobileFriendlyTest;
use google_search_console_api::url_inspection::RequestUrlInspection;


async fn test_token() -> AccessToken {
    // 認証
    let secret = yup_oauth2::read_service_account_key("./test.json")
        .await
        .expect("test.json");
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(secret).build().await.unwrap();
    let scopes = &["https://www.googleapis.com/auth/webmasters"];

    let token = auth.token(scopes).await;
    assert!(token.is_ok(), "{}", token.err().unwrap().to_string());
    token.unwrap()
}


#[tokio::test]
async fn test_sitemaps() {
    let token = test_token().await;
    println!("{:?}", token);
    let res = SearchConsoleApi::url_inspection().inspect(token.as_str(), &RequestUrlInspection {
        site_url: "https://example.com/".to_string(),
        inspection_url: "https://example.com/".to_string(),
        language_code: "ja".to_string(),
    }).await;
    println!("{:?}", res);
    assert!(res.is_ok(), "{:?}", res.err().unwrap().to_string());

}

