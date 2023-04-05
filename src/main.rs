mod types;
use types::*;

fn main() {
    let p = Person::default();
    let m = Message::Person {
        sequence: 1,
        person: p.clone(),
    };
    let json = serde_json::to_string(&m).unwrap();
    println!("json={json}");

    let m2: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(m, m2);
}
