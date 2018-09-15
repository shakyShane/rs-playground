pub fn run() {
    let cmp = compose(one, two);
    let output = cmp(1);
    assert_eq!(output, 4);
}

fn compose<'f, T1, T2, T3, F1, F2>(f1: F1, f2: F2)
       -> Box<Fn(T1) -> T3 + 'f>
    where F1: Fn(T1) -> T2 + 'f,
          F2: Fn(T2) -> T3 + 'f,
{
    Box::new(move |input| f2(f1(input)))
}

fn one(input: usize) -> usize {
    input + 1
}

fn two(input: usize) -> usize {
    input + 2
}