type MwResult = Result<(), String>;
trait Middleware<T> {
    fn before(&self) -> MwResult {
        Ok(())
    }
    fn after(&self) -> MwResult {
        Ok(())
    }
}

struct MyMiddleware1;
struct MyMiddleware2;
struct AppState;

impl Middleware<AppState> for MyMiddleware1 {
    fn before(&self) -> MwResult {
        Err(String::from("Ooops!"))
    }
}

impl Middleware<AppState> for MyMiddleware2 {
    fn before(&self) -> MwResult {
        Ok(())
    }
}

fn get_middlewares<'a>() -> Vec<&'a Middleware<AppState>> {
    vec![&MyMiddleware1 {}, &MyMiddleware2 {}]
}

pub fn run() {
    let mws = get_middlewares();
    let output: Vec<MwResult> = mws.iter().map(|mw| mw.before()).collect();
    assert_eq!(output[0], Err(String::from("Ooops!")));
    assert_eq!(output[1], Ok(()));
}
