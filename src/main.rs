use std::env;
use std::fs::{self, File};
use std::io::{BufReader};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;
use unrar::Archive;

/// 获取解压目标目录（与文件同名的文件夹）
fn get_output_dir(file_path: &str) -> PathBuf {
    let path = Path::new(file_path);
    let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
    path.with_file_name(file_stem.into_owned()) // 生成同名文件夹路径
}

/// 解压 ZIP 文件
fn unzip_file(zip_path: &str) -> zip::result::ZipResult<()> {
    let output_dir = get_output_dir(zip_path);
    fs::create_dir_all(&output_dir)?; // 创建文件夹

    let file = File::open(zip_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = output_dir.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    println!("ZIP 解压成功: {:?}", output_dir);
    Ok(())
}

/// 解压 RAR 文件
fn unrar_file(rar_path: &str) -> Result<(), String> {
    let output_dir = get_output_dir(rar_path);
    fs::create_dir_all(&output_dir).map_err(|e| format!("无法创建目录: {:?}", e))?;

    let archive = Archive::new(rar_path.to_string());
    let entries = archive.extract_to(output_dir.to_str().unwrap().into())
        .map_err(|e| format!("无法打开 RAR 文件: {:?}", e))?;

    for entry in entries {
        entry.map_err(|e| format!("解压失败: {:?}", e))?;
    }

    println!("RAR 解压成功: {:?}", output_dir);
    Ok(())
}

fn main() {
    println!("Welcome to Dog-zip!");

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了文件路径
    if args.len() < 2 {
        eprintln!("用法: {} <压缩文件路径>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1]; // 获取用户传入的文件路径

    if file_path.ends_with(".zip") {
        match unzip_file(file_path) {
            Ok(_) => println!("ZIP 文件解压完成！"),
            Err(e) => eprintln!("ZIP 解压失败: {:?}", e),
        }
    } else if file_path.ends_with(".rar") {
        match unrar_file(file_path) {
            Ok(_) => println!("RAR 文件解压完成！"),
            Err(e) => eprintln!("RAR 解压失败: {}", e),
        }
    } else {
        eprintln!("不支持的文件格式！");
    }
}
