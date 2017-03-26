#[test]
fn it_works() {
    let lv: u64 = 123;
    let rv: u64 = 789;
    assert_eq!(lv, List::to_number(&List::from_number(lv)));
    assert_eq!(
    lv + rv,
    List::to_number(
        &add_two_numbers(
            List::from_number(lv),
            List::from_number(rv)
        )
    ))
}

enum List {
    Cons(u64, Box<List>),
    Nil,
}

impl List {
    fn new_nil() -> List {
        List::Nil
    }

    fn new_cons(v: u64) -> List {
        List::Cons(v, Box::new(List::new_nil()))
    }

    fn to_number(&self) -> u64 {
        match *self {
            List::Cons(v, ref n) => v + 10 * List::to_number(n),
            List::Nil            => 0,
        }
    }

    fn from_number(val: u64) -> List {
        if val <= 9 {
            return List::new_cons(val);
        }
        let (remainder, prev) = (val % 10, val / 10);
        List::Cons(
            remainder,
            Box::new(List::from_number(prev))
        )
    }
}

fn add_two_numbers(l: List, r: List) -> List {
    match (l, r) {
        (List::Cons(lv, ln), List::Cons(rv, rn)) => {
            let sum = lv + rv;
            let (carry, remainder) = (sum / 10, sum % 10);
            List::Cons(
                remainder,
                Box::new(add_two_numbers(
                    List::new_cons(carry),
                    add_two_numbers(*ln, *rn)
                ))
            )
        },
        (lo @ List::Cons(_, _), List::Nil       ) => lo,
        (List::Nil,        ro @ List::Cons(_, _)) => ro,
        (List::Nil,             List::Nil       ) => List::Nil,
    }
}