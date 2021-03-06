use anyhow::Result;

#[tokio::test]
async fn serach_tweets() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;

    let res: kuon::SearchResult = api.search_tweets("rust").await?;
    assert_eq!(res.search_metadata.query, "rust");

    Ok(())
}

#[tokio::test]
async fn serach_tweets_with_params() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = maplit::hashmap! { "count" => "15" };

    let res: kuon::SearchResult = api.search_tweets_with_params("rust", &params).await?;
    assert_eq!(res.search_metadata.query, "rust");
    assert_eq!(res.search_metadata.count, 15);
    assert!(res.statuses[0].text.len() > 0);

    Ok(())
}
