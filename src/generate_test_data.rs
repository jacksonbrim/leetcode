use chrono::Local;
use std::fs;
use std::io::{BufWriter, Result, Write};
use std::path::Path;
use tracing::info;
pub fn generate_number_list(nums: i64, path_name: Option<&String>) -> Result<String> {
    let now = Local::now();

    let path_name = if let Some(path) = path_name {
        path.clone()
    } else {
        format!(
            "./test_input/generated_number_list_{}.txt",
            now.format("%Y-%m-%d_%H-%M-%S")
        )
    };

    let f = fs::File::create(&path_name)?;
    let mut writer = BufWriter::new(f);
    for i in 0..=nums {
        write!(writer, "{},", i)?;
    }
    for i in (1..nums).rev() {
        if i == 1 {
            write!(writer, "{}\n", i)?;
        } else {
            write!(writer, "{},", i)?;
        }
    }
    info!("generated numbers: {}, ... {}, ... {} ", 0, nums, 1);
    let absolute_path = fs::canonicalize(Path::new(&path_name))?;
    let output_fp: String = format!("{}", absolute_path.display());
    Ok(output_fp)
}
