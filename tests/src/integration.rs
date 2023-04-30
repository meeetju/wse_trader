#[cfg(test)]
mod tests {

    // In order to run the test first run the following
    // wse_trader\mock_third_party> cargo run
    // wse_trader\backend> cargo run -- --oa 127.0.0.1 --op 80 --companies-list-url http://127.0.0.1:8765/spolki-rating/akcje_gpw --company-indicators-url http://127.0.0.1:8765/notowania/gpw/

    use serde_json::json;

    #[tokio::test]
    async fn test_search_companies_twice() {
        let client = reqwest::Client::new();

        for _ in 0..2 {
            let response = client
                .get("http://127.0.0.1:80/search_companies")
                .send()
                .await
                .unwrap();
            let content = json!(response.text().await.unwrap());

            let expected = json!(
                r#"[{"name":"MONNARI","ticker":"MON","link":"http://127.0.0.1:8765/notowania/gpw/monnari-mon/wskazniki-finansowe","altman":"AAA","f_score":8.0,"pe":2.2,"roe":26.5,"p_bv":0.58,"p_bvg":1.29},{"name":"XTB","ticker":"XTB","link":"http://127.0.0.1:8765/notowania/gpw/xtb-xtb/wskazniki-finansowe","altman":"AAA","f_score":9.0,"pe":4.86,"roe":50.87,"p_bv":2.47,"p_bvg":2.57}]"#
            );
            assert_eq!(expected, content);
        }
    }
}
