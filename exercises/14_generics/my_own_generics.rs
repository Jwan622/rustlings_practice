
// A test example similar to how Result<T,E> works. 
#[derive(Debug)]
enum Fruit {
    Banana,
    Strawberry,
}

#[derive(Debug)]
enum Veggie {
    Spinach,
    Kale,
}

// generic enum -- like Result, but OK/Err -> Fruit Veggie. one at a time so use enum. This shake type can be FruitBlend holding Fruit OR VeggieBle holding a Veggie.
// similar to: let ok: Result<String, i32> = Ok("hello".to_string());
// uses T = String, never touches E — but type is still Result<String, i32>
enum MilkShake<F, V> {
    FruitBlend(F),
    VeggieBlend(V),
}


fn main() {
    let fruity: MilkShake<Fruit, Veggie> = MilkShake::FruitBlend(Fruit::Banana);
    let veggie: MilkShake<Fruit, Veggie> = MilkShake::VeggieBlend(Veggie::Kale);

    println!("{fruity:?}");
    println!("{veggie:?}");
}


// If you wanted both in one shake
// Use a struct with two fields:

// struct MilkShake<F, V> {
//     fruit: F,
//     veggie: V,
// }
// 
// let combo = MilkShake {
//     fruit: Fruit::Banana,
//     veggie: Veggie::Kale,
// };
// 
// Then you must provide both.

// Yes — if one value must hold fruit and veggie together, a struct (or an enum variant with two fields) is the usual approach.

// Struct — both fields always present
// struct MilkShake<F, V> {
//     fruit: F,
//     veggie: V,
// }
// fn main() {
//     let combo = MilkShake {
//         fruit: Fruit::Banana,
//         veggie: Veggie::Kale,
//     };
//     // MilkShake<Fruit, Veggie> — always has BOTH
// }
// Enum variant with two fields — either/or, but one variant can hold both

// enum MilkShake<F, V> {
//     FruitOnly(F),
//     VeggieOnly(V),
//     Combo { fruit: F, veggie: V },  // both in ONE variant
// }
// let combo = MilkShake::Combo {
//     fruit: Fruit::Banana,
//     veggie: Veggie::Kale,
// };