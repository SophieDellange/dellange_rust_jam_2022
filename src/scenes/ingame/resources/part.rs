//as in monster part
#[derive(Component, PartialEq, Clone)]
pub struct Part{
    connected_parts: Vec<Part>,
    index: usize,
}

pub fn check_valid_init(part_core: Part,parts: &mut Vec<Part>,)-> Vec<Part>{    
    let mut connected = Vec::new();

    //then check for connected parts
    check_valid(part_core,  parts, &mut connected);

    connected
}

pub fn check_valid(current: Part, remaining: &mut Vec<Part>, connecteds: &mut Vec<Part>){
    for part in current.connected_parts.iter(){
        match remaining.iter().position(|element| element == part){
            Some(pos) => Some(connecteds.push(remaining.remove(pos))),
            None => None,
        };

        check_valid(part.clone(),remaining, connecteds);
    }
}

fn test(){
    let mut part_core = Part {connected_parts: Vec::new(), index: 0};

    let mut parts = Vec::new();

    for i in 0..9{
        parts.push(
            Part{
                connected_parts: Vec::new(),
                index: i + 1,
            }
        )
    }

    let result = check_valid_init(part_core, &mut parts);

    for i in result.iter(){
        println!("part {} connected!",i.index);
    }
}