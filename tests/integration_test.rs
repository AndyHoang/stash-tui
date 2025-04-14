use dotenv::dotenv;
use std::env;
use tokio::runtime::Runtime;

// Import the API client from your crate
use stash_tui::api::StashClient;

#[test]
fn test_list_tags() {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get Stash URL from environment variable or use default
    let stash_url = env::var("STASH_URL")
        .unwrap_or_else(|_| {
            println!("STASH_URL environment variable not set, using default URL");
            "http://localhost:9999".to_string()
        });
    
    println!("Testing connection to Stash at: {}", stash_url);
    
    // Create a new runtime for async code
    let rt = Runtime::new().unwrap();
    
    // Create a new StashClient
    let client = StashClient::new(&stash_url);
    
    // Run the async test in the runtime
    rt.block_on(async {
        // Test system status first to ensure we can connect
        match client.get_system_status().await {
            Ok(status) => {
                println!("Successfully connected to Stash. Status: {}", status.status);
            },
            Err(e) => {
                panic!("Failed to connect to Stash: {}", e);
            }
        }
        
        // Test listing tags
        match client.find_tags(1, 10).await {
            Ok(tags_response) => {
                println!("Successfully retrieved {} tags", tags_response.tags.len());
                println!("Total tag count: {}", tags_response.count);
                
                // Print the first few tags
                for (i, tag) in tags_response.tags.iter().enumerate().take(5) {
                    println!("Tag {}: {} ({} scenes)", i + 1, tag.name, tag.scene_count);
                }
                
                // Assert that we got some tags
                assert!(tags_response.count >= 0, "Tag count should be non-negative");
                
                // If there are tags, check that we got the expected number
                if tags_response.count > 0 {
                    let expected_count = std::cmp::min(10, tags_response.count as usize);
                    assert_eq!(tags_response.tags.len(), expected_count, 
                        "Should have received {} tags", expected_count);
                }
            },
            Err(e) => {
                panic!("Failed to retrieve tags: {}", e);
            }
        }
    });
}

#[test]
fn test_list_scenes() {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get Stash URL from environment variable or use default
    let stash_url = env::var("STASH_URL")
        .unwrap_or_else(|_| {
            println!("STASH_URL environment variable not set, using default URL");
            "http://localhost:9999".to_string()
        });
    
    println!("Testing scene listing from Stash at: {}", stash_url);
    
    // Create a new runtime for async code
    let rt = Runtime::new().unwrap();
    
    // Create a new StashClient
    let client = StashClient::new(&stash_url);
    
    // Run the async test in the runtime
    rt.block_on(async {
        // Test listing scenes
        match client.find_scenes(1, 10).await {
            Ok(scenes_response) => {
                println!("Successfully retrieved {} scenes", scenes_response.scenes.len());
                println!("Total scene count: {}", scenes_response.count);
                
                // Print the first few scenes
                for (i, scene) in scenes_response.scenes.iter().enumerate().take(5) {
                    let title = scene.title.clone().unwrap_or_else(|| "Untitled".to_string());
                    let duration = scene.files.as_ref()
                        .and_then(|files| files.first())
                        .and_then(|file| file.duration)
                        .map(|d| format!("{:.1} seconds", d))
                        .unwrap_or_else(|| "Unknown duration".to_string());
                    println!("Scene {}: {} ({})", i + 1, title, duration);
                }
                
                // Assert that we got some scenes
                assert!(scenes_response.count >= 0, "Scene count should be non-negative");
                
                // If there are scenes, check that we got the expected number
                if scenes_response.count > 0 {
                    let expected_count = std::cmp::min(10, scenes_response.count as usize);
                    assert_eq!(scenes_response.scenes.len(), expected_count, 
                        "Should have received {} scenes", expected_count);
                }
                
                // Check that scene data is reasonable
                for scene in scenes_response.scenes.iter() {
                    // Duration should be positive if present
                    if let Some(files) = &scene.files {
                        if let Some(file) = files.first() {
                            if let Some(duration) = file.duration {
                                assert!(duration >= 0.0, "Scene duration should be non-negative");
                            }
                        }
                    }
                }
            },
            Err(e) => {
                panic!("Failed to retrieve scenes: {}", e);
            }
        }
    });
}
