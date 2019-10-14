use std::iter::Iterator;

pub fn main() {
    let all = vec!["aa", "bbb", "cccc"];

    // map
    let res: Vec<_> = all.iter().map(|&x| x.len()).collect();
    assert_eq!(res, vec![2, 3, 4]);

    // filter
    let res: Vec<_> = all.iter().filter(|x| x == &&"bbb").collect();
    assert_eq!(res, vec![&"bbb"]);

    // filter then map
    let res: Vec<_> = all
        .iter()
        .filter(|x| x.len() > 2)
        .map(|x| x.len() * 5)
        .collect();
    assert_eq!(res, vec![15, 20]);

    // filter_map
    let res: Vec<_> = all
        .iter()
        .filter_map(|x| if *x == "bbb" { Some(x.len()) } else { None })
        .collect();
    assert_eq!(res, vec![3]);

    // find_map
    let res = all
        .iter()
        .find_map(|x| if *x == "bbb" { Some(x.len()) } else { None });
    assert_eq!(res, Some(3));

    check_expression(
        &|| {
            all.iter()
                .find_map(|x| if *x == "bbb" { Some(x.len()) } else { None })
        },
        Some(3),
    );

    // group by (partition)
    let (res, res2): (Vec<&str>, Vec<&str>) = all.iter().partition(|&x| *x == "bbb");
    // println!("filter: {:?} -> {:?}", all, res);
    assert_eq!(res, vec!["bbb"]);
    assert_eq!(res2, vec!["aa", "cccc"]);

    // fold
    let res = all.iter().fold(-100, |acc, x| acc + 1);
    assert_eq!(res, -97);
    let char_count = all.iter().fold(0, |acc, x| acc + x.len());
    assert_eq!(char_count, 9);

    check_expression(&|| all.iter().fold(0, |acc, x| acc + x.len()), 9);

    // zip and unzip
    // check_expression(&||all.iter().zip(all).collect(),vec!["aa", "bbb", "cccc"]);

    // flatten
    let twice = vec![all.clone(), all];
    // TODO: Why is this a double ref instead of a single one?
    let res = twice.iter().flatten().collect::<Vec<&&str>>();
    println!("{:?}", res);
    // check_expression(&||twice.iter().flatten().collect(), vec!["aa", "bbb", "cccc"]);

    // let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    // let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    // assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);

    // flatmap

    println!("fin.");
}

fn check_expression<T: PartialEq + std::fmt::Debug>(f: &dyn Fn() -> T, expected: T) {
    let res = f();
    assert_eq!(expected, res);
}
