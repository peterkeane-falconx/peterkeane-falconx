use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

fn main() -> io::Result<()> {
    let codec_target_dir = "rust/sbe";
    let codec_schema_dir = "rust/sbe/schema";
    let sbe_jar = "rust/sbe/sbe-all.jar";

    let md_schema = format!("{}/falconstrike-mds-api.xml", codec_schema_dir);
    let oms_schema = format!("{}/falconstrike-oms-api.xml", codec_schema_dir);

    let generated_codecs = format!("{}/include", codec_target_dir);

    std::process::Command::new("java")
        .arg(format!("-Dsbe.output.dir={}", generated_codecs))
        .arg("-Dsbe.generate.ir=true")
        .arg("-Dsbe.target.language=rust")
        .arg("-Dsbe.xinclude.aware=true")
        .arg("-Dsbe.target.namespace=com.falconstrike.jets.api")
        .arg("-jar")
        .arg(&sbe_jar)
        .arg(&md_schema)
        .status()
        .unwrap();

    std::process::Command::new("java")
        .arg(format!("-Dsbe.output.dir={}", generated_codecs))
        .arg("-Dsbe.generate.ir=true")
        .arg("-Dsbe.target.language=rust")
        .arg("-Dsbe.xinclude.aware=true")
        .arg("-Dsbe.target.namespace=com.falconstrike.jets.api")
        .arg("-jar")
        .arg(&sbe_jar)
        .arg(&oms_schema)
        .status()
        .unwrap();

    let src_mds = "rust/sbe/include/com_falconstrike_jets_api_mds_codec_sbe/src";
    let src_oms = "rust/sbe/include/com_falconstrike_jets_api_oms_codec_sbe/src";
    let dest = PathBuf::from("rust/sbe/include/com_falconstrike_jets_api");

    fs::create_dir_all(&dest)?;

    println!("Source MDS: {:?}", fs::read_dir(src_mds)?);
    println!("Source OMS: {:?}", fs::read_dir(src_oms)?);

    for item in fs::read_dir(src_mds)? {
        let item = item?;
        //println!("Item MDS: {:?}", item.file_type());
        if item.file_type()?.is_file() {
            let dest_path = dest.join(item.file_name());
            //println!("Copying {} to {}", item.path().display(), dest_path.display());
            fs::copy(item.path(), dest_path)?;
        }
    }

    for item in fs::read_dir(src_oms)? {
        let item = item?;
        //println!("Item OMS: {:?}", item.file_type());
        if item.file_type()?.is_file() {
            let dest_path = dest.join(item.file_name());
            //println!("Copying {} to {}", item.path().display(), dest_path.display());
            fs::copy(item.path(), dest_path)?;
        }
    }

    fs::remove_dir_all(src_mds)?;
    fs::remove_dir_all(src_oms)?;

    Ok(())
}
