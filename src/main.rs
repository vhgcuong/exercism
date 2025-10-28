use exercism::knapsack;
use exercism::knapsack::Item;

fn main() {
    let items = [
        Item {
            weight: 5,
            value: 10,
        },
        Item {
            weight: 4,
            value: 40,
        },
        Item {
            weight: 6,
            value: 30,
        },
        Item {
            weight: 4,
            value: 50,
        },
    ];
    knapsack::maximum_value(10, &items);
}
