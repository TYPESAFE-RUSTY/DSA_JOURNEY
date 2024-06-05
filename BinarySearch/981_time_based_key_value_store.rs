use std::collections::HashMap;

struct Data {
    timestamp: i32,
    value: String,
}

struct TimeMap {
    content: HashMap<String, Vec<Data>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            content: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.content.get_mut(&key) {
            Some(vect) => vect.push(Data { timestamp, value }),
            None => {
                self.content
                    .insert(key, Vec::from([Data { value, timestamp }]));
            }
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let mut res: &String = &"".to_string();
        if let Some(values) = self.content.get(&key) {
            let mut left = 0;
            let mut right =( values.len() - 1) as i32;
            while left <= right {
                let mid =( (left + right) / 2) as usize;
                if values[mid].timestamp == timestamp {
                    return values[mid].value.clone();
                } else if values[mid].timestamp < timestamp {
                    res = &values[mid].value;
                    left = mid as i32 + 1;
                } else {
                    right = mid as i32 - 1;
                }
            }
            return res.to_string();
        }
        "".to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */