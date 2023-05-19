use std::io::{self, Read};

enum LookFor {
    Tag,
    Name,
    Key,
    QuotedValue,
    Value,
}

struct Node {
    pub id: u64,
    pub version: u16,
    pub timestamp: String,
    pub lat: f32,
    pub lon: f32,
}

struct Tag {
    pub k: String,
    pub v: String,
}

struct Way {
    pub id: u64,
    pub version: u16,
    pub timestamp: String,
}

struct Nd {
    pub r#ref: u64,
}

struct Relation {
    pub id: u64,
    pub version: u16,
    pub timestamp: String,
}

struct Member {
    pub r#type: String,
    pub r#ref: u64,
    pub role: String,
}

fn main() {
    let mut buffer = [0; 10_000];
    let mut stdin = io::stdin();
    let mut size;

    let mut mode = LookFor::Tag;
    let mut name = String::new();
    let mut key = String::new();
    let mut value = String::new();

    // Initialize entities
    let mut node = Node {
        id: 0,
        version: 0,
        timestamp: "".to_string(),
        lat: 0.0,
        lon: 0.0,
    };
    let mut tag = Tag {
        k: "".to_string(),
        v: "".to_string(),
    };
    let mut way = Way {
        id: 0,
        version: 0,
        timestamp: "".to_string(),
    };
    let mut nd = Nd {
        r#ref: 0,
    };
    let mut relation = Relation {
        id: 0,
        version: 0,
        timestamp: "".to_string(),
    };
    let mut member = Member {
        r#type: "".to_string(),
        r#ref: 0,
        role: "".to_string(),
    };

    // Parse
    loop {
        size = stdin.read(&mut buffer).unwrap();
        if size == 0 { break; }

        buffer[..size].iter().for_each(|&byte| {
            match (&mode, byte) {
                // Tag start
                (LookFor::Tag, b'<') => {
                    name.clear();
                    mode = LookFor::Name;
                },
                (LookFor::Tag, _) => {},
                // Tag end
                (LookFor::Key, b'>' | b'/') => { mode = LookFor::Tag; },
                // Closing tag or XML declaration
                (LookFor::Name, b'/' | b'?') => { mode = LookFor::Tag; },
                // Looking for key
                (LookFor::Name | LookFor::Key, b' ') => {
                    key.clear();
                    mode = LookFor::Key;
                },
                // Parse tag name
                (LookFor::Name, _) => { name.push(byte as char); },
                // Parse attributes
                (LookFor::Key, b'a'..=b'z' | b'_') => { key.push(byte as char); },
                (LookFor::Key, b'=') => { mode = LookFor::QuotedValue; },
                (LookFor::QuotedValue, b'"') => {
                    value.clear();
                    mode = LookFor::Value;
                },
                // Parse/store value
                (LookFor::Value, b'"') => {
                    match (name.as_str(), key.as_str()) {
                        // Node
                        ("node", "id") => { node.id = value.parse::<u64>().unwrap(); },
                        ("node", "version") => { node.version = value.parse::<u16>().unwrap(); },
                        ("node", "timestamp") => { node.timestamp = value.clone(); },
                        ("node", "lat") => { node.lat = value.parse::<f32>().unwrap(); },
                        ("node", "lon") => { node.lon = value.parse::<f32>().unwrap(); },
                        ("tag", "k") => { tag.k = value.clone(); },
                        ("tag", "v") => { tag.v = value.clone(); },
                        // Way
                        ("way", "id") => { way.id = value.parse::<u64>().unwrap(); },
                        ("way", "version") => { way.version = value.parse::<u16>().unwrap(); },
                        ("way", "timestamp") => { way.timestamp = value.clone(); },
                        // Nd
                        ("nd", "ref") => { nd.r#ref = value.parse::<u64>().unwrap(); },
                        // Relation
                        ("relation", "id") => { relation.id = value.parse::<u64>().unwrap(); },
                        ("relation", "version") => { relation.version = value.parse::<u16>().unwrap(); },
                        ("relation", "timestamp") => { relation.timestamp = value.clone(); },
                        // Member
                        ("member", "type") => { member.r#type = value.clone(); },
                        ("member", "ref") => { member.r#ref = value.parse::<u64>().unwrap(); },
                        ("member", "role") => { member.role = value.clone(); },
                        // Bounding box
                        ("bound", _) => { },
                        // Changeset
                        ("changeset", _) => { },
                        // OSM root element
                        ("osm", _) => { },
                        (_, _) => {
                            panic!("tag {} key {}", name.as_str(), key.as_str());
                        },
                    }
                    mode = LookFor::Key;
                },
                (LookFor::Value, _) => { value.push(byte as char); },
                // Panic otherwise
                _ => {
                    println!("{}", buffer[..size].iter().map(|i| *i as char).collect::<String>());
                    panic!("{:?}", byte as char);
                },
            }
        });
    }
}
