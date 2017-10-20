pub fn join_string(list: &[String], seq: &str) -> String {
    list.iter().fold(String::new(), |a, b| if a.is_empty() { a } else { a + seq } + b)
}

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { eprintln!("Error: {}", e); return; }
        }
    }}
);
