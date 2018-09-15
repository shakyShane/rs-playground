type MwResult = Result<(), String>;
trait Middleware {
    fn before(&self) -> MwResult {
        Ok(())
    }
    fn after(&self) -> MwResult {
        Ok(())
    }
}

struct MyMiddleware;

impl Middleware for MyMiddleware {
    fn before(&self) -> MwResult {
        Err(String::from("Ooops!"))
    }
}

pub fn run () {
    let v: Vec<&Middleware> = vec![
        &MyMiddleware{}
    ];

    for m in v.iter() {
        let output = m.before();
        println!("{:?}", output);
    }
}