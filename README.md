# starrailrust

fetch & parse everything from [https://github.com/Mar-7th/StarRailRes/tree/master/index_min/{LANGUAGE}](https://github.com/Mar-7th/StarRailRes)


## example usage:

first, add to cargo.toml dependencies
```
starrailrust = { git = "https://github.com/yuvlian/starrailrust" }
```

then main.rs

```rust
use starrailrust::{BaseUrl, Client, character::CharacterMap};

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let url = BaseUrl::En; // English
    let client = Client::new(); // a reqwest client
    
    // returns self or err
    let character_map = CharacterMap::fetch_map(&url, &client).await?; 

    // returns vec String
    println!("Character ids: {:?}\n", character_map.get_keys()); 

    // return Character or none
    let march_data = character_map.get_value_by_key("1001").unwrap();
    println!("{:?}", march_data);
    Ok(())
}
```

for other types of data just look around the code
