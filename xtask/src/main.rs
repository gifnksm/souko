use cli_xtask::{
    config::{ConfigBuilder, DistConfigBuilder},
    workspace, Result, Xtask,
};
use souko::Souko;

fn main() -> Result<()> {
    <Xtask>::main_with_config(|| {
        let workspace = workspace::current();
        let (dist, package) = DistConfigBuilder::from_root_package(workspace)?;
        let command = Souko::command();
        let target = package
            .binary_by_name(command.get_name())?
            .command(command)
            .build()?;
        let dist = dist.package(package.target(target).build()?).build()?;
        let config = ConfigBuilder::new().dist(dist).build()?;
        Ok(config)
    })
}
