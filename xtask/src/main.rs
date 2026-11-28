use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "xtask", about = "Rust-native workspace automation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run workspace tests and check SSR + WASM hydrate targets
    Check,
    /// Sync content/ (excluding .search_index) into deploy/content/
    SyncContent,
    /// Package deploy/ and deploy to remote server via SSH (reads .env)
    Deploy {
        /// Skip syncing content/ into deploy/content/ before uploading
        #[arg(long)]
        no_sync: bool,
        /// Use system `tar` binary instead of the built-in Rust tar.gz archiver
        #[arg(long)]
        use_system_tar: bool,
    },
    /// Run server-side installation & systemd setup (replaces setup.sh)
    Setup {
        #[arg(long)]
        domain: Option<String>,
        #[arg(long)]
        user: Option<String>,
        #[arg(long)]
        target_dir: Option<String>,
        #[arg(long, default_value = "deploy")]
        bundle_dir: PathBuf,
        #[arg(long)]
        systemd: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let root = workspace_root();

    let res = match cli.command {
        Commands::Check => run_check(&root),
        Commands::SyncContent => sync_content(&root),
        Commands::Deploy {
            no_sync,
            use_system_tar,
        } => run_deploy(&root, no_sync, use_system_tar),
        Commands::Setup {
            domain,
            user,
            target_dir,
            bundle_dir,
            systemd,
        } => {
            let domain = domain
                .or_else(|| env::var("DOMAIN").ok())
                .unwrap_or_else(|| "irshadali.org".to_string());
            let user = user
                .or_else(|| env::var("SUDO_USER").ok())
                .or_else(|| env::var("USER").ok())
                .unwrap_or_else(|| "irshad".to_string());
            let target_dir = target_dir.or_else(|| env::var("TARGET_DIR").ok());
            run_setup(&domain, &user, target_dir.as_deref(), &bundle_dir, systemd)
        }
    };

    if let Err(err) = res {
        eprintln!("xtask error: {err}");
        std::process::exit(1);
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live inside workspace root")
        .to_path_buf()
}

fn run_cmd(mut cmd: Command, desc: &str) -> Result<(), String> {
    let status = cmd
        .status()
        .map_err(|e| format!("failed to execute {desc}: {e}"))?;
    if !status.success() {
        return Err(format!("{desc} exited with status {status}"));
    }
    Ok(())
}

fn run_check(root: &Path) -> Result<(), String> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let mut test_cmd = Command::new(&cargo);
    test_cmd.current_dir(root).args(["test", "--workspace"]);
    run_cmd(test_cmd, "cargo test --workspace")?;

    let mut hydrate_cmd = Command::new(&cargo);
    hydrate_cmd
        .current_dir(root)
        .args(["check", "-p", "ui", "--features", "hydrate"]);
    run_cmd(hydrate_cmd, "cargo check -p ui --features hydrate")?;

    let mut ssr_cmd = Command::new(&cargo);
    ssr_cmd
        .current_dir(root)
        .args(["check", "-p", "app", "--features", "ssr"]);
    run_cmd(ssr_cmd, "cargo check -p app --features ssr")?;

    println!("All checks and tests passed.");
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<usize, String> {
    fs::create_dir_all(dst).map_err(|e| format!("create {}: {e}", dst.display()))?;
    let mut count = 0;

    for entry in fs::read_dir(src).map_err(|e| format!("read {}: {e}", src.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = entry.file_name();
        if name == ".search_index" {
            continue;
        }
        let target = dst.join(&name);
        if path.is_dir() {
            count += copy_dir_recursive(&path, &target)?;
        } else if path.is_file() {
            fs::copy(&path, &target)
                .map_err(|e| format!("copy {} -> {}: {e}", path.display(), target.display()))?;
            count += 1;
        }
    }

    Ok(count)
}

fn sync_content(root: &Path) -> Result<(), String> {
    let src = root.join("content");
    let dst = root.join("deploy").join("content");
    let count = copy_dir_recursive(&src, &dst)?;
    println!("Synced {count} files from {} to {}", src.display(), dst.display());
    Ok(())
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(stripped);
        }
    }
    PathBuf::from(path)
}

fn append_ustar_entry(
    out: &mut Vec<u8>,
    rel_path: &str,
    data: &[u8],
    is_dir: bool,
    mode: u32,
) -> Result<(), String> {
    let mut header = [0u8; 512];
    let name_bytes = rel_path.as_bytes();
    if name_bytes.len() > 100 {
        return Err(format!("tar path exceeds 100 bytes: {rel_path}"));
    }
    header[..name_bytes.len()].copy_from_slice(name_bytes);
    header[100..108].copy_from_slice(format!("{mode:07o}\0").as_bytes());
    header[108..116].copy_from_slice(b"0000000\0");
    header[116..124].copy_from_slice(b"0000000\0");
    header[124..136].copy_from_slice(format!("{:011o}\0", data.len()).as_bytes());
    header[136..148].copy_from_slice(b"00000000000\0");
    header[148..156].fill(b' ');
    header[156] = if is_dir { b'5' } else { b'0' };
    header[257..263].copy_from_slice(b"ustar\0");
    header[263..265].copy_from_slice(b"00");

    let checksum: u32 = header.iter().map(|&b| b as u32).sum();
    header[148..156].copy_from_slice(format!("{checksum:06o}\0 ").as_bytes());

    out.extend_from_slice(&header);
    if !data.is_empty() {
        out.extend_from_slice(data);
        let rem = data.len() % 512;
        if rem != 0 {
            out.resize(out.len() + (512 - rem), 0);
        }
    }
    Ok(())
}

fn collect_tar_entries(base: &Path, dir: &Path, raw_tar: &mut Vec<u8>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| format!("read {}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let rel = path
            .strip_prefix(base)
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        if path.is_dir() {
            append_ustar_entry(raw_tar, &format!("{rel}/"), &[], true, 0o755)?;
            collect_tar_entries(base, &path, raw_tar)?;
        } else if path.is_file() {
            let data = fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
            #[cfg(unix)]
            let mode = {
                use std::os::unix::fs::PermissionsExt;
                fs::metadata(&path).map(|m| m.permissions().mode() & 0o777).unwrap_or(0o644)
            };
            #[cfg(not(unix))]
            let mode = 0o644;
            append_ustar_entry(raw_tar, &rel, &data, false, mode)?;
        }
    }
    Ok(())
}

fn create_tar_gz(dir: &Path) -> Result<Vec<u8>, String> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    let mut raw_tar = Vec::new();
    collect_tar_entries(dir, dir, &mut raw_tar)?;
    raw_tar.resize(raw_tar.len() + 1024, 0);

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&raw_tar)
        .map_err(|e| format!("gzip compress: {e}"))?;
    encoder.finish().map_err(|e| format!("gzip finish: {e}"))
}

fn run_deploy(root: &Path, no_sync: bool, use_system_tar: bool) -> Result<(), String> {
    let env_path = root.join(".env");
    if env_path.exists() {
        let _ = dotenvy::from_path(&env_path);
    }

    if !no_sync {
        sync_content(root)?;
    }

    let domain = env::var("DOMAIN").unwrap_or_else(|_| "irshadali.org".to_string());
    let server_ip = env::var("IP_ADDRESSE").unwrap_or_else(|_| "20.198.120.162".to_string());
    let ssh_user = env::var("USER").unwrap_or_else(|_| "irshad".to_string());
    let ssh_key_raw =
        env::var("KEY").unwrap_or_else(|_| "~/secure/queos-server_key.pem".to_string());
    let ssh_key = expand_tilde(&ssh_key_raw);
    let remote_dir = format!("~/deploy_{}", domain.replace('.', "_"));

    println!("==================================================");
    println!(" Deploying {domain} to Remote Server");
    println!(" Host: {ssh_user}@{server_ip}");
    println!(" Key:  {}", ssh_key.display());
    println!("==================================================");

    if !ssh_key.exists() {
        return Err(format!("SSH key not found at {}", ssh_key.display()));
    }

    let target = format!("{ssh_user}@{server_ip}");
    let ssh_key_str = ssh_key.to_string_lossy().to_string();
    let base_ssh_args = [
        "-o",
        "StrictHostKeyChecking=no",
        "-o",
        "ConnectTimeout=10",
        "-i",
        &ssh_key_str,
    ];

    println!("[1/4] Verifying SSH connectivity...");
    let mut ping_cmd = Command::new("ssh");
    ping_cmd
        .args(base_ssh_args)
        .args([&target, "echo '[+] SSH Connection established successfully'"]);
    run_cmd(ping_cmd, "SSH connectivity check")?;

    println!("[2/4] Packaging and uploading build bundle...");
    let mut mkdir_cmd = Command::new("ssh");
    mkdir_cmd
        .args(base_ssh_args)
        .args([&target, &format!("mkdir -p {remote_dir}")]);
    run_cmd(mkdir_cmd, "remote mkdir")?;

    let deploy_dir = root.join("deploy");
    if use_system_tar {
        let mut tar_child = Command::new("tar")
            .args(["-czf", "-", "-C"])
            .arg(&deploy_dir)
            .arg(".")
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to spawn tar: {e}"))?;

        let tar_stdout = tar_child
            .stdout
            .take()
            .ok_or_else(|| "failed to capture tar stdout".to_string())?;

        let mut untar_cmd = Command::new("ssh");
        untar_cmd
            .args(base_ssh_args)
            .args([&target, &format!("tar -xzf - -C {remote_dir}")])
            .stdin(tar_stdout);
        run_cmd(untar_cmd, "remote untar")?;
        let tar_status = tar_child.wait().map_err(|e| e.to_string())?;
        if !tar_status.success() {
            return Err(format!("local tar failed with {tar_status}"));
        }
    } else {
        use std::io::Write;
        let archive = create_tar_gz(&deploy_dir)?;
        let mut untar_cmd = Command::new("ssh")
            .args(base_ssh_args)
            .args([&target, &format!("tar -xzf - -C {remote_dir}")])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to spawn ssh untar: {e}"))?;
        if let Some(mut stdin) = untar_cmd.stdin.take() {
            stdin
                .write_all(&archive)
                .map_err(|e| format!("write archive to ssh stdin: {e}"))?;
        }
        let status = untar_cmd.wait().map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!("remote untar failed with {status}"));
        }
    }

    println!("[3/4] Running remote server setup...");
    let remote_setup = format!(
        "cd {remote_dir} && sudo bash -c 'TARGET_DIR=/var/www/{domain}; mkdir -p $TARGET_DIR/{{data,content,site,data/pingap}}; \
        cp server $TARGET_DIR/server && chmod +x $TARGET_DIR/server; \
        cp -r site/* $TARGET_DIR/site/ && cp -r content/* $TARGET_DIR/content/; \
        cp -r data/* $TARGET_DIR/data/ 2>/dev/null || true; \
        [ -f dev_site.db ] && cp dev_site.db $TARGET_DIR/ || true; \
        cp pingap.toml $TARGET_DIR/pingap.toml; \
        sed -i \"s/User=irshad/User={ssh_user}/g\" dev-site.service; \
        if [ \"{domain}\" != \"irshadali.org\" ]; then sed -i \"s/irshadali.org/{domain}/g\" dev-site.service pingap.service $TARGET_DIR/pingap.toml; fi; \
        cp dev-site.service /etc/systemd/system/dev-site.service && cp pingap.service /etc/systemd/system/pingap.service; \
        systemctl daemon-reload && systemctl enable dev-site.service pingap.service && systemctl restart dev-site.service pingap.service'"
    );
    let mut setup_cmd = Command::new("ssh");
    setup_cmd
        .args(base_ssh_args)
        .args(["-t", &target, &remote_setup]);
    run_cmd(setup_cmd, "remote setup")?;

    println!("[4/4] Verifying deployment...");
    let mut verify_cmd = Command::new("ssh");
    verify_cmd
        .args(base_ssh_args)
        .args([&target, "curl -sI http://127.0.0.1:3000/ || true"]);
    run_cmd(verify_cmd, "remote verification")?;

    println!("==================================================");
    println!("🎉 Deployment successfully finished!");
    println!("==================================================");
    Ok(())
}

fn run_setup(
    domain: &str,
    user: &str,
    target_dir: Option<&str>,
    bundle_dir: &Path,
    install_systemd: bool,
) -> Result<(), String> {
    let target = target_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(format!("/var/www/{domain}")));

    for sub in ["data", "content", "site", "data/pingap"] {
        fs::create_dir_all(target.join(sub))
            .map_err(|e| format!("create {}: {e}", target.join(sub).display()))?;
    }

    let server_bin = bundle_dir.join("server");
    if server_bin.exists() {
        fs::copy(&server_bin, target.join("server"))
            .map_err(|e| format!("copy server binary: {e}"))?;
    }
    let db_file = bundle_dir.join("dev_site.db");
    if db_file.exists() {
        fs::copy(&db_file, target.join("dev_site.db"))
            .map_err(|e| format!("copy dev_site.db: {e}"))?;
    }
    for dir in ["site", "content", "data"] {
        let src = bundle_dir.join(dir);
        if src.exists() {
            copy_dir_recursive(&src, &target.join(dir))?;
        }
    }

    let pingap = bundle_dir.join("pingap.toml");
    if pingap.exists() {
        let mut content = fs::read_to_string(&pingap).map_err(|e| e.to_string())?;
        if domain != "irshadali.org" {
            content = content.replace("irshadali.org", domain);
        }
        fs::write(target.join("pingap.toml"), content).map_err(|e| e.to_string())?;
    }

    if install_systemd {
        for (src_name, dst_path) in [
            ("dev-site.service", "/etc/systemd/system/dev-site.service"),
            ("pingap.service", "/etc/systemd/system/pingap.service"),
        ] {
            let src = bundle_dir.join(src_name);
            if src.exists() {
                let mut unit = fs::read_to_string(&src).map_err(|e| e.to_string())?;
                unit = unit.replace("User=irshad", &format!("User={user}"));
                if domain != "irshadali.org" {
                    unit = unit.replace("irshadali.org", domain);
                }
                fs::write(dst_path, unit).map_err(|e| format!("write {dst_path}: {e}"))?;
            }
        }

        if Command::new("systemctl").arg("--version").output().is_ok() {
            let mut reload = Command::new("systemctl");
            reload.arg("daemon-reload");
            run_cmd(reload, "systemctl daemon-reload")?;

            for svc in ["dev-site.service", "pingap.service"] {
                let mut en = Command::new("systemctl");
                en.args(["enable", svc]);
                run_cmd(en, &format!("systemctl enable {svc}"))?;

                let mut rs = Command::new("systemctl");
                rs.args(["restart", svc]);
                run_cmd(rs, &format!("systemctl restart {svc}"))?;
            }
        } else {
            eprintln!("Warning: systemctl not available; unit files written without reloading.");
        }
    }

    println!("Setup files prepared at {}", target.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let expanded = expand_tilde("~/.ssh/id_ed25519");
        assert!(!expanded.to_string_lossy().starts_with("~/"));
    }

    #[test]
    fn test_create_tar_gz() {
        let dir = env::temp_dir().join("xtask_tar_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("sub/hello.txt"), b"hello rust tar").unwrap();
        let gz = create_tar_gz(&dir).unwrap();
        assert!(gz.starts_with(&[0x1f, 0x8b]));
        let _ = fs::remove_dir_all(&dir);
    }
}
