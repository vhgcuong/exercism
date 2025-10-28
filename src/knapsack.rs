use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {

    let mut results: Vec<Vec<Item>> = vec![];
    for (index, item) in items.iter().enumerate() {
        let remaining_slice = &items[index+1..];
        let last = if remaining_slice.len() > 1 { remaining_slice.len() - 1 } else { 0 };
        let mut sum: u32 = item.weight;
        let mut data: Vec<Item> = vec![item.clone()];
        for (index2, item2) in remaining_slice.iter().enumerate() {
            println!("{:?}", item2);
            sum += item2.weight;
            
            match sum.cmp(&max_weight)  {
                Ordering::Less => {
                    data.push(item2.clone());
                    if index2 == last {
                        results.push(data.clone());
                    }
                },
                Ordering::Equal => {
                    data.push(item2.clone());
                    sum = item.weight;
                    results.push(data.clone());
                    data = vec![item.clone()];
                },
                Ordering::Greater => {
                    sum = item.weight;
                    results.push(data.clone());
                    data = vec![item.clone()];
                }
            }
        }
    }

    results.iter().map(|items| {
        items.iter().map(|item| {
            item.value
        }).sum()
    }).max().unwrap()
}
