use std::path::Path;
use std::env;
use std::io;

fn print_error_lines<P: AsRef<Path>, F: AsRef<Path>>(dir: P, filename: F) -> io::Result<()> {
    // 你的函数实现
    println!("处理目录: {:?}, 文件: {:?}", dir.as_ref(), filename.as_ref());
    Ok(())
}

fn main() {
    // 从命令行参数获取输入
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("用法: {} <目录> <文件名>", args[0]);
        std::process::exit(1);
    }
    
    let dir = &args[1];
    let filename = &args[2];
    
    // 调用函数
    if let Err(e) = print_error_lines(dir, filename) {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
