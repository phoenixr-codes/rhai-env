use rhai::{packages::Package, Engine, EvalAltResult};
use rhai_env::EnvironmentPackage;

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Register our filesystem package.
    let package = EnvironmentPackage::new();
    package.register_into_engine_as(&mut engine, "env");

    std::env::set_var("FOO", "hello");

    engine.run(r#"print(env::envs())"#)?;

    Ok(())
}
