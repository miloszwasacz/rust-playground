fn main() {
    infinite_loop();
    println!();
    labels_and_returns();
    println!();
    for_loops();
}

fn infinite_loop() {
    println!("Infinite");

    let mut count = 0u8;
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{count}");

        if count == 5 {
            println!("Ok, that's enough");
            break;
        }
    }
}

#[allow(unreachable_code, unused_labels, clippy::never_loop)]
fn labels_and_returns() {
    println!("Labels & returning");
    'outer: loop {
        println!("Outer loop");

        'inner: loop {
            println!("Inner loop");

            break 'outer;
        }

        println!("After inner loop (should be unreachable)");
    }

    {
        let mut counter = 0;
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
        println!("Result = {result} (should be 20)");
    }
}

fn for_loops() {
    println!("For loops");
    for_range();
    for_iter();
}

fn for_range() {
    let end = 5;
    println!("End = {end}");
    print!("Exclusive end: ");
    for i in 0..end {
        print!("{i} ");
    }
    print!("\nInclusive end: ");
    for i in 0..=end {
        print!("{i} ");
    }
    println!();
}

fn for_iter() {
    // region: .iter()
    {
        println!("\n.iter()");
        let names = vec!["Alice", "Bob", "Ferris"];
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean here!"),
                _ => println!("Hello {name}"),
            }
        }
        println!("names: {names:?}");
    }
    // endregion
    // region: .into_iter()
    {
        println!("\n.into_iter()");
        let names = vec!["Alice", "Bob", "Ferris"];
        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean here!"),
                _ => println!("Hello {name}"),
            }
        }
        // println!("names: {names:?}");
        // ^ Can't do
    }
    // endregion
    // region: .iter_mut()
    {
        println!("\n.iter_mut()");
        let mut names = vec!["Alice", "Bob", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean here!",
                _ => name,
            }
        }
        println!("names: {names:?}");
    }
    // endregion
}
