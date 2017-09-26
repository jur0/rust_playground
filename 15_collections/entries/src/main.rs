use std::collections::HashMap;

fn main() {
    let mut hmap: HashMap<i32, char> = HashMap::new();

    hmap.insert(1, 'a');
    hmap.insert(2, 'b');
    hmap.insert(3, 'c');
    hmap.insert(4, 'd');

    assert_eq!(get_or_insert_entry1(&mut hmap, 5, 'e'), Some(&'e'));
    assert_eq!(get_or_insert_entry1(&mut hmap, 5, 'e'), Some(&'e'));

    assert_eq!(get_or_insert_entry2(&mut hmap, 5, 'e'), Some(&'e'));
    assert_eq!(get_or_insert_entry2(&mut hmap, 6, 'f'), Some(&'f'));
    assert_eq!(get_or_insert_entry1(&mut hmap, 7, 'g'), Some(&'g'));
}

// Without entries, hashmap is accessed 2/3 times (in the worst case).
fn get_or_insert_entry1(hmap: &mut HashMap<i32, char>, k: i32, v: char) ->
    Option<&char>
{
    // 1st access.
    if !hmap.contains_key(&k) {
        // 2nd access.
        hmap.insert(k, v);
    }
    // 2/3rd access.
    hmap.get(&k)
}

fn get_or_insert_entry2(hmap: &mut HashMap<i32, char>, k: i32, v: char) ->
    Option<&char>
{
    // Just one access using entry.
    Some(hmap.entry(k).or_insert(v))
}
