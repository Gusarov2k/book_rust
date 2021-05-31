fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Так как if является выражением (а значит возвращает значение),
// то его можно использовать в правой части кода оператора let

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {}", number);
// }


// Циклы

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }


// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// Более кратко

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }

// Вот так может выглядеть реализация обратного отсчёта с
// применением цикла for и метода rev который изменит последовательность диапазона на обратную

// fn main() {
//     for number in (1..4).rev() {
//         println!("{}!", number);
//     }
//     println!("LIFTOFF!!!");
// }
