#[cfg(feature = "dircpy_fork")]
use dircpy_fork::CopyBuilder;
#[cfg(feature = "dircpy_upstream")]
use dircpy_upstream::CopyBuilder;

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::try_init()?;

    let release =
        ureq::get("https://nodejs.org/dist/v20.17.0/node-v20.17.0-linux-x64.tar.xz").call()?;
    let archive = liblzma::decode_all(release.into_reader())?;
    tar::Archive::new(archive.as_slice()).unpack("meow")?;

    CopyBuilder::new("meow", "node").overwrite(true).run()?;

    Ok(())
}
