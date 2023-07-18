use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, QueryStringValue<'buffer>>,
}

#[derive(Debug)]
pub enum QueryStringValue<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&QueryStringValue> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut QueryStringValue| match existing {
                    QueryStringValue::Single(prev_val) => {
                        *existing = QueryStringValue::Multiple(vec![prev_val, val]);
                    }
                    QueryStringValue::Multiple(vec) => vec.push(val),
                })
                .or_insert(QueryStringValue::Single(val));
        }

        QueryString { data }
    }
}
