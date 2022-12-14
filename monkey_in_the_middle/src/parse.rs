fn parse_monkey(
    [_monkey_id, items, operation, test_divisor, test_true, test_false]: [&str; 6],
) -> Option<crate::Monkey> {
    let items = {
        let (_, items) = items.split_once(':')?;

        let items = items
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .map(Result::ok);

        items.collect::<Option<Vec<_>>>()?
    };

    let operation = {
        let (_, operation) = operation.split_once('=')?;

        let mut tokens = operation.trim().split_ascii_whitespace();
        let left = tokens.next()?;
        let operation = tokens.next()?;
        let right = tokens.next()?;

        let lhs = match left {
            "old" => crate::Value::Old,
            _ => crate::Value::Lit {
                value: left.parse().ok()?,
            },
        };
        let rhs = match right {
            "old" => crate::Value::Old,
            _ => crate::Value::Lit {
                value: right.parse().ok()?,
            },
        };

        match operation {
            "+" => crate::Operation::Add { lhs, rhs },
            "*" => crate::Operation::Mul { lhs, rhs },
            _ => return None,
        }
    };

    let test = {
        let divisor = test_divisor.split_ascii_whitespace().last()?.parse().ok()?;
        let true_monkey = test_true.split_ascii_whitespace().last()?.parse().ok()?;
        let false_monkey = test_false.split_ascii_whitespace().last()?.parse().ok()?;

        crate::Test {
            divisor,
            true_monkey,
            false_monkey,
        }
    };

    Some(crate::Monkey {
        items,
        operation,
        test,

        inspection_count: 0,
    })
}

pub fn parse(input: &str) -> Option<crate::Game> {
    let monkeys = input
        .lines()
        .filter(|line| !line.is_empty())
        .array_chunks::<6>()
        .map(parse_monkey)
        .collect::<Option<Vec<_>>>()?;
    let moves = Vec::with_capacity(64);

    let max_worry = monkeys.iter().map(|monkey| monkey.test.divisor).product();

    Some(crate::Game {
        monkeys,
        moves,

        max_worry,
    })
}
