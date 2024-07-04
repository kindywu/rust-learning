#![allow(unused)]

fn main() {
    let age = 32;
    test(age);
    // test(&age);

    static AGE: u32 = 32;
    test(AGE);
    test(&AGE);

    let msg = "name".to_string();
    test(msg);
    // test(&msg);

    let user = User {
        name: "kindy".to_string(),
    };
    test(user);

    let level = Level { name: "IC-4" };
    test(level);

    // let level_name = "IC-4".to_string();
    // let level = Level { name: &level_name };
    // test(level);

    static LEVEL_NAME: &str = "IC-4";
    let level = Level { name: LEVEL_NAME };
    test(level);
}

fn test<T: 'static>(_t: T) {}

struct User {
    name: String,
}

struct Level<'a> {
    name: &'a str,
}
