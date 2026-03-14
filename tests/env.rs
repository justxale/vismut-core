use vismut_core::ExecutionEnvironment;
use vismut_core::RegistryError;
#[test]
fn new_registry() -> Result<(), RegistryError> {
    let env = ExecutionEnvironment::new();
    Ok(())
}

#[test]
#[cfg(feature = "nodes")]
fn default_registry() -> Result<(), RegistryError> {
    let env = ExecutionEnvironment::default();
    Ok(())
}
