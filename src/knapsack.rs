use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    if items.len() == 0 {
        return 0
    }

    let mut results: Vec<Vec<Item>> = vec![];
    for (index, item) in items.iter().enumerate() {
        let mut sum: u32 = item.weight;

        if sum > max_weight {
            continue;
        }

        if sum == max_weight {
            results.push(vec![item.clone()]);
            continue;
        }

        let mut data: Vec<Item> = vec![item.clone()];
        let last = if items.len() > 1 { items.len() - 1 } else { 0 };
        for (index2, item2) in items.iter().enumerate() {
            if index2 == index {
                continue;
            }

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

    if results.len() == 0 {
        return 0;
    }

    let max = results.iter().map(|items| {
        items.iter().map(|item| {
            item.value
        }).sum()
    }).max().unwrap();

    results.iter().for_each(|items| {
        println!("{:?}", items);
    });

    max
}
