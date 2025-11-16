use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::env;

/// 进入指定目录后读取指定文件，打印包含 "error" 的行（区分大小写）
///
/// # 参数
/// - dir: 目录路径
/// - filename: 文件名或相对路径（在切换到 dir 之后解析）
///
/// # 返回
/// - io::Result<()>: 出错会返回 Err
pub fn print_error_lines<P: AsRef<Path>, F: AsRef<Path>>(dir: P, filename: F) -> io::Result<()> {
    // 切换当前工作目录
    env::set_current_dir(dir)?;

    // 打开文件
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    // 逐行读取并打印包含 "error" 的行
    for line_res in reader.lines() {
        let line = line_res?;
        if line.contains("error") {
            println!("{}", line);
        }
    }

    Ok(())
}