use exercism::knapsack;
use exercism::knapsack::Item;

fn main() {
    let max_weight = 104;
    let items = [
        Item {
            weight: 25,
            value: 350,
        },
        Item {
            weight: 35,
            value: 400,
        },
        Item {
            weight: 45,
            value: 450,
        },
        Item {
            weight: 5,
            value: 20,
        },
        Item {
            weight: 25,
            value: 70,
        },
        Item {
            weight: 3,
            value: 8,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
    ];

    knapsack::maximum_value(max_weight, &items);
}
