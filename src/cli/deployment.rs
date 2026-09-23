//! Read-only deployment planning and capability-relative execution.
//!
//! The CLI-selected root is trusted. Below it, each directory is opened without
//! following links, and all operations use directory handles, never logged paths.
//! This is not a transaction or an evaluator sandbox. The output tree must not
//! be writable by an adversary (including one able to rename open directories).

use super::options::CliOptions;
use cap_fs_ext::{AccessType, DirExt, FollowSymlinks, OpenOptionsFollowExt};
use cap_primitives::fs::AccessModes;
use cap_std::fs::{Dir, File, Metadata, OpenOptions};
use cap_tempfile::TempFile;
use std::ffi::OsString;
use std::io::{self, Seek, Write};
use std::path::{Component, Path, PathBuf};

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

const WRITE_ACCESS: AccessType = AccessType::Access(AccessModes {
    readable: false,
    writable: true,
    executable: false,
});
const READ_ACCESS: AccessType = AccessType::Access(AccessModes {
    readable: true,
    writable: false,
    executable: false,
});

/// Enforce the same portable generated-path policy on every host. CLI root
/// paths, unlike generated paths, may be absolute and use native separators.
fn relative_path(path: &str) -> io::Result<PathBuf> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains(['\\', ':'])
        || path.chars().any(char::is_control)
        || path.ends_with('/')
    {
        return Err(invalid(format!(
            "Unsafe output path {path:?}: use a relative file path with forward slashes; select an absolute base with --root"
        )));
    }
    let mut result = PathBuf::new();
    for part in path.split('/') {
        if part == ".." {
            return Err(invalid(format!("Path traversal detected: {path:?}")));
        }
        if part.is_empty() || part == "." {
            continue;
        }
        let stem = part.split('.').next().unwrap_or_default().to_uppercase();
        let device = matches!(
            stem.as_str(),
            "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$"
        ) || ["COM", "LPT"].iter().any(|prefix| {
            stem.strip_prefix(prefix).is_some_and(|suffix| {
                matches!(
                    suffix,
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                )
            })
        });
        if part.ends_with(['.', ' ']) || part.contains(['<', '>', '"', '|', '?', '*']) || device {
            return Err(invalid(format!("Non-portable output path: {path:?}")));
        }
        result.push(part);
    }
    if result.as_os_str().is_empty()
        || !result
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
    {
        return Err(invalid(format!("Expected a relative file path: {path:?}")));
    }
    Ok(result)
}

struct Root {
    anchor: Dir,
    missing: PathBuf,
    absolute: PathBuf,
}

impl Root {
    /// Resolve the nearest existing ancestor without creating missing roots.
    fn open(root: Option<&str>) -> io::Result<Self> {
        let mut ancestor = std::env::current_dir()?.join(root.unwrap_or("."));
        let mut missing = Vec::<OsString>::new();
        loop {
            match std::fs::symlink_metadata(&ancestor) {
                Ok(_) => break,
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    let name = ancestor.file_name().ok_or_else(|| {
                        invalid("Cannot resolve output root; avoid '..' after missing directories")
                    })?;
                    missing.push(name.to_owned());
                    ancestor.pop();
                }
                Err(e) => return Err(e),
            }
        }
        // The caller intentionally chooses this ambient root, including aliases.
        // All generated paths below the opened capability are untrusted.
        let canonical = std::fs::canonicalize(&ancestor)?;
        let anchor = Dir::open_ambient_dir(&canonical, cap_std::ambient_authority())?;
        let missing: PathBuf = missing.into_iter().rev().collect();
        let absolute = if missing.as_os_str().is_empty() {
            canonical
        } else {
            canonical.join(&missing)
        };
        Ok(Self {
            anchor,
            missing,
            absolute,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Create,
    Overwrite,
    Append,
    Skip,
}

impl Action {
    fn label(self) -> &'static str {
        match self {
            Self::Create => "CREATE",
            Self::Overwrite => "OVERWRITE",
            Self::Append => "APPEND",
            Self::Skip => "SKIP (exists)",
        }
    }
}

struct Entry<'a> {
    relative: PathBuf,
    content: &'a str,
    action: Action,
    backup: bool,
}

/// Open one component at a time so even links within the root are rejected.
/// With create=false, stop at the nearest existing directory (read-only).
fn directory_at(start: &Dir, path: &Path, create: bool) -> io::Result<(Dir, bool)> {
    let mut dir = start.try_clone()?;
    for component in path.components() {
        let Component::Normal(name) = component else {
            return Err(invalid("Invalid deployment directory component"));
        };
        match dir.symlink_metadata(name) {
            Ok(meta) if meta.file_type().is_symlink() || !meta.is_dir() => {
                return Err(invalid(format!(
                    "Unsafe output directory {name:?}: symlinks and non-directories are not allowed"
                )));
            }
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                if !create {
                    return Ok((dir, false));
                }
                match dir.create_dir(name) {
                    Ok(()) => {}
                    // Another writer may have created it; the nofollow open below
                    // still checks the type and refuses a substituted symlink.
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
        dir = dir.open_dir_nofollow(name)?;
    }
    Ok((dir, true))
}

fn file_metadata(dir: &Dir, name: &Path) -> io::Result<Option<Metadata>> {
    match dir.symlink_metadata(name) {
        Ok(meta) if !meta.is_file() || meta.file_type().is_symlink() => Err(invalid(format!(
            "Unsafe output file {name:?}: symlinks, directories, and special files are not allowed"
        ))),
        Ok(meta) => Ok(Some(meta)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

fn backup_name(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".bak");
    PathBuf::from(name)
}

fn check_writable(dir: &Dir, name: &Path, meta: &Metadata) -> io::Result<()> {
    if meta.permissions().readonly() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("Cannot write read-only file {name:?}"),
        ));
    }
    dir.access(name, WRITE_ACCESS)
}

fn plan<'a>(
    root: &Root,
    files: &'a [(String, String)],
    opts: &CliOptions,
) -> io::Result<Vec<Entry<'a>>> {
    let mut entries = Vec::new();
    let mut targets = Vec::new();
    for (path, content) in files {
        let relative = relative_path(path)?;
        let anchored = root.missing.join(&relative);
        let (parent, exists) = directory_at(&root.anchor, anchored.parent().unwrap(), false)?;
        let name = Path::new(relative.file_name().unwrap());
        let meta = if exists {
            file_metadata(&parent, name)?
        } else {
            None
        };
        let action = match meta {
            None => Action::Create,
            Some(_) if opts.if_not_exists => Action::Skip,
            Some(_) if opts.append => Action::Append,
            Some(_) if opts.force || opts.backup => Action::Overwrite,
            Some(_) => Action::Skip,
        };
        let backup = meta.is_some() && opts.backup && action != Action::Skip;
        if action != Action::Skip {
            // Read-only access checks only: never use write probes or backup placeholders.
            parent.access(".", WRITE_ACCESS)?;
            if let Some(meta) = &meta {
                check_writable(&parent, name, meta)?;
                if backup || action == Action::Append {
                    parent.access(name, READ_ACCESS)?;
                }
            }
            if backup {
                let backup = backup_name(name);
                if let Some(meta) = file_metadata(&parent, &backup)? {
                    check_writable(&parent, &backup, &meta)?;
                }
            }
        }
        // Reject duplicates, file/directory conflicts, and backup collisions,
        // conservatively ignoring case for portable plans on Windows/macOS.
        targets.push(relative.clone());
        if backup {
            targets.push(backup_name(&relative));
        }
        entries.push(Entry {
            relative,
            content,
            action,
            backup,
        });
    }
    let mut keys: Vec<PathBuf> = targets
        .iter()
        .map(|p| PathBuf::from(p.to_string_lossy().to_lowercase()))
        .collect();
    keys.sort();
    for (i, path) in keys.iter().enumerate() {
        if keys[..i].iter().any(|other| path.starts_with(other)) {
            return Err(invalid(format!(
                "Conflicting output or backup paths: {path:?}"
            )));
        }
    }
    Ok(entries)
}

fn read_existing(parent: &Dir, name: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).follow(FollowSymlinks::No);
    // A concurrent FIFO substitution must not hang the process during open.
    #[cfg(unix)]
    {
        use cap_std::fs::OpenOptionsExt;
        options.custom_flags(rustix::fs::OFlags::NONBLOCK.bits() as i32);
    }
    let file = parent.open_with(name, &options)?;
    if !file.metadata()?.is_file() {
        return Err(invalid(
            "Output changed into a special file during deployment",
        ));
    }
    Ok(file)
}

fn replace_file(
    parent: &Dir,
    name: &Path,
    content: &[u8],
    append: bool,
    backup: bool,
) -> io::Result<()> {
    let meta = file_metadata(parent, name)?
        .ok_or_else(|| invalid("Output disappeared during deployment"))?;
    check_writable(parent, name, &meta)?;
    let mut staged = TempFile::new(parent)?;
    // Set permissions before writing sensitive contents. Never transfer Unix
    // setuid/setgid bits to newly generated content.
    let permissions = meta.permissions();
    #[cfg(unix)]
    let permissions = {
        use cap_std::fs::PermissionsExt;
        cap_std::fs::Permissions::from_mode(permissions.mode() & 0o777)
    };
    staged.as_file().set_permissions(permissions.clone())?;
    let mut original = if append || backup {
        Some(read_existing(parent, name)?)
    } else {
        None
    };
    if backup {
        let backup_path = backup_name(name);
        if let Some(meta) = file_metadata(parent, &backup_path)? {
            check_writable(parent, &backup_path, &meta)?;
        }
        let mut saved = TempFile::new(parent)?;
        saved.as_file().set_permissions(permissions)?;
        io::copy(original.as_mut().unwrap(), &mut saved)?;
        saved.as_file().sync_all()?;
        saved.replace(backup_path.as_os_str())?;
        original.as_mut().unwrap().rewind()?;
    }
    if append {
        io::copy(original.as_mut().unwrap(), &mut staged)?;
    }
    staged.write_all(content)?;
    staged.as_file().sync_all()?;
    // Close source handles before replacement (important on Windows).
    drop(original);
    // Rename replaces a directory entry, never follows a substituted final link.
    staged.replace(name.as_os_str())
}

/// Public within the CLI so the REPL uses exactly the same safety rules.
pub(crate) fn deploy(files: &[(String, String)], opts: &CliOptions) -> i32 {
    let prepared = Root::open(opts.root.as_deref()).and_then(|root| {
        let entries = plan(&root, files, opts)?;
        Ok((root, entries))
    });
    let (root, entries) = match prepared {
        Ok(plan) => plan,
        Err(e) => {
            eprintln!("Error: {e}\nDeployment aborted. No files were written.");
            return 1;
        }
    };
    if opts.dry_run {
        println!("Deployment plan (dry run)");
        println!(
            "Root: {}{}",
            root.absolute.display(),
            if opts.root.is_none() {
                " (current working directory)"
            } else {
                ""
            }
        );
        for entry in &entries {
            if entry.backup {
                println!(
                    "BACKUP {} -> {}",
                    root.absolute.join(&entry.relative).display(),
                    root.absolute.join(backup_name(&entry.relative)).display()
                );
            }
            println!(
                "{} {}",
                entry.action.label(),
                root.absolute.join(&entry.relative).display()
            );
        }
        println!("No files or directories were created or changed by deployment.");
        println!("Read-only checks only; write access, available space, and filesystem state may change before deployment.");
        return 0;
    }
    let mut written = 0;
    for entry in entries {
        let display = if opts.root.is_some() {
            root.absolute.join(&entry.relative)
        } else {
            entry.relative.clone()
        };
        if entry.action == Action::Skip {
            if opts.if_not_exists {
                println!("Skipped {} (exists)", display.display());
            } else {
                eprintln!("WARNING: File {} exists. Use --force to overwrite, --append to append, or --backup to backup and overwrite.", display.display());
            }
            continue;
        }
        let result = (|| -> io::Result<()> {
            let anchored = root.missing.join(&entry.relative);
            let (parent, _) = directory_at(&root.anchor, anchored.parent().unwrap(), true)?;
            let name = Path::new(entry.relative.file_name().unwrap());
            if entry.action == Action::Create {
                // Atomic exclusive creation prevents a concurrent file/link from
                // being overwritten. No predictable preflight probe is needed.
                let mut file =
                    parent.open_with(name, OpenOptions::new().write(true).create_new(true))?;
                file.write_all(entry.content.as_bytes())?;
                file.sync_all()?;
            } else {
                replace_file(
                    &parent,
                    name,
                    entry.content.as_bytes(),
                    entry.action == Action::Append,
                    entry.backup,
                )?;
            }
            Ok(())
        })();
        if let Err(e) = result {
            eprintln!("Error: Failed to deploy {}: {e}", display.display());
            eprintln!("Deployment aborted. {written} file(s) completed; directories, backups, or a partial new file may remain. No rollback was performed.");
            return 1;
        }
        if entry.backup {
            println!("Backed up to {}", backup_name(&display).display());
        }
        let verb = match entry.action {
            Action::Create => "Wrote",
            Action::Append => "Appended to",
            _ => "Overwrote",
        };
        println!("{verb} {}", display.display());
        written += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn options(path: &Path) -> CliOptions {
        let mut opts = CliOptions::new();
        opts.root = Some(path.to_str().unwrap().to_string());
        opts
    }

    fn files(paths: &[&str]) -> Vec<(String, String)> {
        paths
            .iter()
            .map(|p| (p.to_string(), "new content".to_string()))
            .collect()
    }

    #[test]
    fn portable_paths_reject_absolute_traversal_devices_and_log_controls() {
        for path in [
            "",
            ".",
            "./",
            "../x",
            "a/../x",
            "/tmp/x",
            "C:/x",
            "C:x",
            "\\\\host\\share",
            "//host/share",
            "a\\b",
            "a:stream",
            "a\nCREATE victim",
            "a\0b",
            "NUL",
            "aux.txt",
            "COM1",
            "lpt².log",
            "trailing.",
            "trailing ",
            "a?b",
            "dir/",
        ] {
            assert!(relative_path(path).is_err(), "accepted {path:?}");
        }
        assert_eq!(
            relative_path("./config//app.txt").unwrap(),
            Path::new("config/app.txt")
        );
        assert!(relative_path(".config/hello world.txt").is_ok());
    }

    #[test]
    fn dry_run_missing_root_has_no_side_effects() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("missing/deep");
        let mut opts = options(&root);
        opts.dry_run = true;
        assert_eq!(deploy(&files(&["nested/app.txt"]), &opts), 0);
        assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 0);
    }

    #[test]
    fn invalid_later_path_does_not_create_root() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("missing");
        assert_eq!(
            deploy(&files(&["okay.txt", "../escape"]), &options(&root)),
            1
        );
        assert!(!root.exists());
    }

    #[test]
    fn collisions_include_normalization_case_ancestors_and_backups() {
        let temp = tempfile::tempdir().unwrap();
        for paths in [
            vec!["a", "./a"],
            vec!["A", "a"],
            vec!["a", "a/b"],
            vec!["a/b", "a"],
        ] {
            assert_eq!(deploy(&files(&paths), &options(temp.path())), 1);
        }
        fs::write(temp.path().join("a"), "original").unwrap();
        let mut opts = options(temp.path());
        opts.backup = true;
        assert_eq!(deploy(&files(&["a", "a.bak"]), &opts), 1);
        assert!(!temp.path().join("a.bak").exists());
        assert_eq!(
            fs::read_to_string(temp.path().join("a")).unwrap(),
            "original"
        );
    }

    #[test]
    fn invalid_later_file_never_truncates_a_backup_or_probe() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("a"), "original").unwrap();
        fs::write(temp.path().join("a.bak"), "old backup").unwrap();
        fs::write(temp.path().join(".avon_write_test"), "unrelated").unwrap();
        fs::create_dir(temp.path().join("blocked")).unwrap();
        let mut opts = options(temp.path());
        opts.backup = true;
        assert_eq!(deploy(&files(&["a", "blocked"]), &opts), 1);
        assert_eq!(
            fs::read_to_string(temp.path().join("a.bak")).unwrap(),
            "old backup"
        );
        assert_eq!(
            fs::read_to_string(temp.path().join(".avon_write_test")).unwrap(),
            "unrelated"
        );
        assert_eq!(
            fs::read_to_string(temp.path().join("a")).unwrap(),
            "original"
        );
    }

    #[test]
    fn actions_and_backup_append_precedence_match_execution() {
        let temp = tempfile::tempdir().unwrap();
        let mut opts = options(temp.path());
        let inputs = files(&["a"]);
        assert_eq!(deploy(&inputs, &opts), 0);
        let root = Root::open(opts.root.as_deref()).unwrap();
        assert_eq!(plan(&root, &inputs, &opts).unwrap()[0].action, Action::Skip);
        opts.backup = true;
        opts.append = true;
        let entry = &plan(&root, &inputs, &opts).unwrap()[0];
        assert_eq!(entry.action, Action::Append);
        assert!(entry.backup);
        opts.dry_run = true;
        assert_eq!(deploy(&inputs, &opts), 0);
        assert!(!temp.path().join("a.bak").exists());
        assert_eq!(
            fs::read_to_string(temp.path().join("a")).unwrap(),
            "new content"
        );
        opts.dry_run = false;
        assert_eq!(deploy(&inputs, &opts), 0);
        assert_eq!(
            fs::read_to_string(temp.path().join("a")).unwrap(),
            "new contentnew content"
        );
        assert_eq!(
            fs::read_to_string(temp.path().join("a.bak")).unwrap(),
            "new content"
        );
        opts.if_not_exists = true;
        let entry = &plan(&root, &inputs, &opts).unwrap()[0];
        assert_eq!(entry.action, Action::Skip);
        assert!(!entry.backup);
    }

    #[test]
    fn hard_linked_files_and_backups_do_not_modify_other_names() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("outside"), "keep me").unwrap();
        fs::hard_link(temp.path().join("outside"), temp.path().join("a")).unwrap();
        fs::hard_link(temp.path().join("outside"), temp.path().join("a.bak")).unwrap();
        let mut opts = options(temp.path());
        opts.backup = true;
        opts.append = true;
        assert_eq!(deploy(&files(&["a"]), &opts), 0);
        assert_eq!(
            fs::read_to_string(temp.path().join("outside")).unwrap(),
            "keep me"
        );
        assert_eq!(
            fs::read_to_string(temp.path().join("a")).unwrap(),
            "keep menew content"
        );
        assert_eq!(
            fs::read_to_string(temp.path().join("a.bak")).unwrap(),
            "keep me"
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_in_output_and_backups_are_rejected_even_with_missing_descendants() {
        use std::os::unix::fs::symlink;
        let temp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        symlink(outside.path(), temp.path().join("link")).unwrap();
        symlink(outside.path().join("missing"), temp.path().join("dangling")).unwrap();
        fs::write(temp.path().join("a"), "original").unwrap();
        symlink(outside.path().join("backup"), temp.path().join("a.bak")).unwrap();
        let mut opts = options(temp.path());
        opts.backup = true;
        for dry_run in [true, false] {
            opts.dry_run = dry_run;
            for path in [
                "link/file",
                "link/missing/deep/file",
                "dangling",
                "dangling/sub/file",
                "a",
            ] {
                assert_eq!(deploy(&files(&[path]), &opts), 1, "accepted {path}");
            }
        }
        assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
        assert_eq!(
            fs::read_to_string(temp.path().join("a")).unwrap(),
            "original"
        );
    }

    #[cfg(unix)]
    #[test]
    fn directory_substitution_after_planning_is_rejected() {
        use std::os::unix::fs::symlink;
        let temp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        fs::create_dir(temp.path().join("nested")).unwrap();
        let opts = options(temp.path());
        let root = Root::open(opts.root.as_deref()).unwrap();
        let inputs = files(&["nested/file"]);
        assert!(plan(&root, &inputs, &opts).is_ok());
        fs::remove_dir(temp.path().join("nested")).unwrap();
        symlink(outside.path(), temp.path().join("nested")).unwrap();
        assert!(directory_at(&root.anchor, Path::new("nested"), true).is_err());
        assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn permissions_preserved_without_setuid_bits() {
        use std::os::unix::fs::PermissionsExt;
        let temp = tempfile::tempdir().unwrap();
        let file = temp.path().join("a");
        fs::write(&file, "old").unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o6750)).unwrap();
        let mut opts = options(temp.path());
        opts.force = true;
        assert_eq!(deploy(&files(&["a"]), &opts), 0);
        assert_eq!(
            fs::metadata(&file).unwrap().permissions().mode() & 0o7777,
            0o750
        );
    }

    #[test]
    fn empty_plan_does_not_create_root() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("unused");
        assert_eq!(deploy(&[], &options(&root)), 0);
        assert!(!root.exists());
    }
}
