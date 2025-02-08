use eyre::Result;
use bye_pcd_rs::DynReader;

fn main() -> Result<()> {
    let reader = DynReader::open("test_files/binary.pcd")?;
    let points: Result<Vec<_>, _> = reader.collect();
    println!("{} points", points?.len());
    Ok(())
}
