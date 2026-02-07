use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use bioinformatics::{random_number};

fn read_parameters_from_file(filename: String) -> Vec<String> {
    let contents = File::open(filename).expect("Failed to read file");
    let reader = BufReader::new(contents);
    let mut results = Vec::new();
    
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for entry in parts.iter() {
                    results.push(entry.to_string());
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
    results
}

fn convert_parameters_to_hashmap(inputs: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut last_key = "".to_string();
    for entry in inputs {
        if entry.ends_with(':') {
            last_key = entry.to_string();
        } else {
            map.entry(last_key.clone()).or_insert(Vec::new()).push(entry.clone());
        }
    }
    let mut new_map = HashMap::new();
    for (key, value) in map.into_iter() {
        // Remove last character safely
        let new_key = if key.len() > 0 {
            key[..key.len() - 1].to_string()
        } else {
            key // Keep empty keys unchanged
        };
        new_map.insert(new_key, value);
    }
    new_map
}

fn build_cycle(mut map: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut path: Vec<String>= Vec::new();
    let mut keys_with_exit:Vec<String> = Vec::new();
    let mut keys: Vec<String> = Vec::new();
    let mut no_of_keys;
    let mut start_key: String; 
    //path.push(start_key.clone());
    loop {
        keys = map.keys().cloned().collect();
        no_of_keys = map.len();
        start_key = keys[random_number(no_of_keys)].to_string();
        println!("Start Key = {:?}", start_key);

        if let Some(next_nodes) = map.get_mut(&start_key) {    
            if next_nodes.is_empty() {
                break;
            }
            let node_length = next_nodes.len();
            let idx = random_number(node_length);
            let new_start_key = next_nodes.remove(idx); // removes and returns owned String
            if node_length == 1 {
                map.remove(&start_key);
            }
            path.push(new_start_key.clone());
            start_key = new_start_key;
        } else {
            println!("Map before exit: {:?}", &map);
            if map.is_empty() {
                break;
            }
        }
        
        println!("path map {:?} {:?} ", &path, &map);
    }
    path
}

#[test]
fn test_read_hashmap() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/testset.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    println!("map ={:?}", map);
    assert_eq!(1,2);
}

#[test]
fn test_build_cycle() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/testset.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = build_cycle(map);
    assert_eq!(1,2);
}

