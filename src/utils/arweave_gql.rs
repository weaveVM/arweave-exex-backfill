use crate::utils::constants::IRYS_GQL_GATEWAY;
use crate::utils::wvm_client::get_latest_block_number;
use anyhow::Error;
use reqwest::Client;
use serde_json::{json, Value};

async fn send_graphql(gateway: &str, query: Value) -> Result<Value, Error> {
    let client = Client::new();
    let res = client
        .post(format!("{}/graphql", gateway))
        .header("Content-Type", "application/json")
        .json(&query)
        .send()
        .await?;

    let json_res: Value = res.json().await?;
    Ok(json_res)
}

async fn retrieve_all_transactions(scan_count: u32) -> Result<Vec<u32>, Error> {
    let mut block_numbers: Vec<u32> = Vec::new();
    let mut cursor: Option<String> = None;
    const PAGE_SIZE: u32 = 1000;
    let mut page_count: u32 = 0;

    loop {
        page_count += 1;
        println!(
            "Fetching page {}. Current cursor: {:#?}",
            page_count, cursor
        );

        let query = json!({
            "query": r#"
            query GetTransactions($cursor: String, $pageSize: Int!) {
                transactions(
                    first: $pageSize,
                    after: $cursor,
                    order: DESC,
                    tags: [
                        { name: "Protocol", values: ["WeaveVM-ExEx"] }
                    ],
                    owners: ["5JUE58yemNynRDeQDyVECKbGVCQbnX7unPrBRqCPVn5Z", "F8XVrMQzsHiWfn1CaKtUPxAgUkATXQjXULWw3oVXCiFV"]
                ) {
                    edges {
                        node {
                            tags {
                                name
                                value
                            }
                        }
                        cursor
                    }
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                }
            }
            "#,
            "variables": {
                "cursor": cursor,
                "pageSize": PAGE_SIZE
            }
        });

        let res = send_graphql(IRYS_GQL_GATEWAY, query).await?;

        let transactions = res
            .get("data")
            .and_then(|data| data.get("transactions"))
            .ok_or_else(|| Error::msg("Invalid response structure"))?;

        let new_block_numbers: Vec<u32> = transactions
            .get("edges")
            .and_then(|edges| edges.as_array())
            .ok_or_else(|| Error::msg("Edges not found or not an array"))?
            .iter()
            .filter_map(|edge| {
                edge.get("node")
                    .and_then(|node| node.get("tags"))
                    .and_then(|tags| tags.as_array())
                    .and_then(|tags| {
                        tags.iter().find_map(|tag| {
                            if tag.get("name")?.as_str()? == "Block-Number" {
                                tag.get("value")?.as_str()?.parse::<u32>().ok()
                            } else {
                                None
                            }
                        })
                    })
            })
            .collect();

        println!(
            "Fetched {} new block numbers on page {}",
            new_block_numbers.len(),
            page_count
        );
        block_numbers.extend(new_block_numbers);

        let page_info = transactions
            .get("pageInfo")
            .ok_or_else(|| Error::msg("PageInfo not found"))?;

        let has_next_page = page_info
            .get("hasNextPage")
            .and_then(|hnp| hnp.as_bool())
            .unwrap_or(false);

        if !has_next_page {
            println!("No more pages. Pagination complete.");
            break;
        }

        cursor = page_info
            .get("endCursor")
            .and_then(|ec| ec.as_str())
            .map(String::from);

        println!(
            "Page {} complete. Has next page: {}. Next cursor: {:#?}",
            page_count, has_next_page, cursor
        );
        if page_count > scan_count {
            break;
        }
    }

    println!("Pagination complete. Total pages fetched: {}", page_count);

    block_numbers.sort();
    block_numbers.dedup();

    println!("{}", "#".repeat(100));
    println!("Total scanned block numbers: {}", block_numbers.len());

    Ok(block_numbers)
}

pub async fn detect_missing_blocks(scan_count: u32) -> Result<Vec<u32>, Error> {
    // retrieve WeaveVM-ExEx data protocol blocks on Arweave
    let blocks = retrieve_all_transactions(scan_count).await.unwrap();
    // latest WeaveVM block number from the RPC
    let latest_block = get_latest_block_number().await? as u32;

    // expected (correct but not a must to be found) blocks sequencer on Arweave
    // its final form will represent the missed_blocks Vec
    let mut canonical_chain_blocks: Vec<u32> = (0..=latest_block).collect();

    for &current_block in blocks.iter() {
        // Remove current_block from canonical_chain_blocks
        canonical_chain_blocks.retain(|&block| block != current_block);
    }

    println!("missing blocks number: {:#?}", canonical_chain_blocks.len());
    println!("{:#?}", canonical_chain_blocks);

    Ok(canonical_chain_blocks)
}
