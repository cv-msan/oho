use std::env;
use std::process::Command;

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("init-dev") => init_dev(),
        _ => {
            eprintln!("用法: cargo run --package xtask -- init-dev");
            eprintln!("可用任务:");
            eprintln!(" init-dev   初始化开发环境");
        }
    }
}

fn init_dev() {
    println!("🚀 初始化 oho 开发环境...");

    // 1. 激活 git hooks
    println!("\n⚙️  激活 git hooks...");
    run("git", &["config", "core.hooksPath", ".githooks"]);

    // 2. 设置 hooks 执行权限（Linux/Mac 需要，Windows 忽略错误）
    #[cfg(not(target_os = "windows"))]
    {
        run("chmod", &["+x", ".githooks/pre-commit"]);
        run("chmod", &["+x", ".githooks/commit-msg"]);
    }

    println!("✅ git hooks 已激活");

    // 3. 安装 git-cliff
    println!("\n⚙️  安装 git-cliff...");
    if command_exists("git-cliff") {
        println!("✅ git-cliff 已安装，跳过");
    } else {
        run("cargo", &["install", "git-cliff"]);
        println!("✅ git-cliff 安装完成");
    }

    // 4. 安装 cargo-release
    println!("\n⚙️  安装 cargo-release...");
    if command_exists("cargo-release") {
        println!("✅ cargo-release 已安装，跳过");
    } else {
        run("cargo", &["install", "cargo-release"]);
        println!("✅ cargo-release 安装完成");
    }

    println!("\n🎉 初始化完成！");
    println!("\n接下来你可以：");
    println!("  git commit -m 'feat(auth): 添加登录功能'");
    println!("  cargo release minor --dry-run");
}

fn run(cmd: &str, args: &[&str]) {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("执行 {} 失败: {}", cmd, e));

    if !status.success() {
        eprintln!("❌ 命令执行失败: {} {:?}", cmd, args);
        std::process::exit(1);
    }
}

fn command_exists(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().is_ok()
}
