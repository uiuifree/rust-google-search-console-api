## features
Search Console Api  
https://developers.google.com/webmaster-tools/v1/api_reference_index

## Using
```
[dependencies]
google-search-console-api="0.1"
```

## Installation
```
cargo add google-search-console-api
cargo add yup-oauth2
```

## Token Lifetime Example
Using yup_oauth2
``` rust
async fn token() -> AccessToken {
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
```


## runReport
``` rust
let token = token().await;
#[tokio::test]
async fn test_sitemaps() {
    let test_site = "https://example.com/";
    let token = test_token().await;
    println!("{:?}",token);
    let res = SearchConsoleApi::sitemaps().submit(token.as_str(),test_site,"https://example.com/sitemap/static.xml").await;
    assert!(res.is_ok(),"{:?}",res.err().unwrap().to_string());

    let res = SearchConsoleApi::sitemaps().list(token.as_str(),test_site).await;
    assert!(res.is_ok(),"{:?}",res.err());

    let sitemap =res.unwrap();
    assert_ne!(sitemap.sitemap.len(),0);

    for row in sitemap.sitemap{
        let path = row.path.unwrap_or_default();
        let res = SearchConsoleApi::sitemaps().get(token.as_str(),test_site,path.as_str()).await;
        assert!(res.is_ok(),"{:?}",res.err());
        println!("{:?}",res.unwrap())
    }
    let res = SearchConsoleApi::sitemaps().delete(token.as_str(),test_site,"https://example.com/sitemap/static.xml").await;
    assert!(res.is_ok(),"{:?}",res.err());

}

```