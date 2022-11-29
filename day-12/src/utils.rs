use std::collections::HashMap;

pub fn format_input(input: Vec<String>) -> HashMap<String, HashMap<String, bool>> {
    let mut map: HashMap<String, HashMap<String, bool>> = Default::default();
    for i in input {
        let mut split_iter = i.split("-");
        let (a, b) = (split_iter.next().unwrap().to_string(), split_iter.next().unwrap().to_string());
        for [key, value] in [[&a, &b], [&b, &a]] {
            while None == map.get(key) {
                map.insert(String::from(key), HashMap::new());
            }
            let record = map.get_mut(key).unwrap();
            record.insert(String::from(value), true);
            let owned = record.to_owned();
            map.insert(String::from(key), owned);
        }
    }
    return map;
}