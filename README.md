# starrailrust

fetch & parse everything from [https://github.com/Mar-7th/StarRailRes/tree/master/index_min/{LANGUAGE}](https://github.com/Mar-7th/StarRailRes)


## example usage:

first, add to cargo.toml dependencies
```
starrailrust = { git = "https://github.com/yuvlian/starrailrust" }
```

then main.rs

```rust
use starrailrust::{BaseUrl, Client, character::CharacterList};

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let url = BaseUrl::En; // English
    let client = Client::new(); // a reqwest client

    let character_map = CharacterList::fetch_map(&base_url, &client).await?; // returns self or err

    println!("Character ids: {:?}", character_map.get_keys()); // returns vec String

    let march_data = character_map.get_value_by_key("1001"); // return character or none

    println!("{:?}", march_data);
}
```