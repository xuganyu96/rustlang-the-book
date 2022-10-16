use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<String, i32> = HashMap::new();
    /* HashMap::insert() will take ownership of both key and value. */
    prices.insert(String::from("Apple"), 100);
    prices.insert(String::from("Banana"), 200);

    /* To read the value of key, use the "get" method, which returns
     * Option<&V>
     */
    let keys: [&str; 3] = ["Apple", "Banana", "Cherry"];
    for key in keys {
        match prices.get(key) {  // the "get" method returns Option<&V>
            /* price has type &i32 instead of i32 */
            Some(price) => println!("Price of {} is {}", key, price),
            None => println!("Price not found for {}", key),
        }
    }
    /* Use copied() to change Option<&V> type to Option<V> type;
     * use unwrap_or() to set default values for handling Option::None
     */
    for key in keys {
        println!("Price of {} is {}", key, prices.get(key).copied().unwrap_or(0));
    }

    /* iterate through a hashmap using a for-loop */
    for (k, v) in &prices {  // returns references if iterating over reference
        let _k: &String = k;
        let _v: &i32 = v;
        println!("{}, {}", _k, _v);
    }

    /* There are a number of ways to update the value of a key in a hash map */
    let mut stocks: HashMap<String, i32> = HashMap::new();
    stocks.insert(String::from("GME"), 2);
    // 1. Calling HashMap::insert will overwrite any existing values
    stocks.insert(String::from("GME"), 40);
    // 2. HashMap::entry().or_insert() will return a mutable reference to the
    // value if the key exists, or create a new item with the key, then return
    // a mutable reference...
    let amc: &mut i32 = stocks.entry(String::from("AMC")).or_insert(6);
    *amc += 100; // this is mutating the value within the hashmap
}

