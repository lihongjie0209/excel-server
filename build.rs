use std::process::Command;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=documentation/");
    
    // 检查是否已经有构建好的 dist 目录
    if Path::new("documentation/.vitepress/dist/index.html").exists() {
        println!("cargo:warning=Documentation already built, skipping build...");
        return;
    }
    
    // 构建 VitePress 文档
    println!("cargo:warning=Building VitePress documentation...");
    
    // Windows 使用 npm.cmd，Unix 使用 npm
    let npm_command = if cfg!(windows) { "npm.cmd" } else { "npm" };
    
    let status = Command::new(npm_command)
        .args(&["run", "docs:build"])
        .current_dir("documentation")
        .status();
    
    match status {
        Ok(status) if status.success() => {
            println!("cargo:warning=Documentation built successfully");
        }
        Ok(status) => {
            println!("cargo:warning=Documentation build failed with exit code: {:?}", status.code());
            println!("cargo:warning=Continuing without embedded documentation...");
        }
        Err(e) => {
            println!("cargo:warning=Failed to run npm: {}", e);
            println!("cargo:warning=Continuing without embedded documentation...");
        }
    }
}
