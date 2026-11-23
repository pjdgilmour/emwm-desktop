// SPDX-License-Identifier: GPL-3.0-only
//! Include the complete sources and license texts of linked Rust crates in
//! the desktop's source archive. Cargo's resolved graph selects the versions
//! and activated features for each shipped binary or extension.
use super::{capture, run, Result};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Source {
    id: String,
    directory: PathBuf,
    license: Value,
}

fn field<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value[name]
        .as_str()
        .ok_or_else(|| format!("Campo ausente em cargo metadata: {name}").into())
}

fn collect(metadata: &Value, sources: &mut BTreeMap<String, Source>) -> Result<()> {
    let resolve = &metadata["resolve"];
    let nodes: HashMap<_, _> = resolve["nodes"]
        .as_array()
        .ok_or("cargo metadata: resolve.nodes")?
        .iter()
        .map(|node| Ok((field(node, "id")?, node)))
        .collect::<Result<_>>()?;
    let mut needed = HashSet::new();
    let mut pending = vec![field(resolve, "root")?];
    while let Some(id) = pending.pop() {
        if !needed.insert(id) {
            continue;
        }
        let node = nodes
            .get(id)
            .ok_or("cargo metadata: nó de dependência ausente")?;
        for dependency in node["deps"].as_array().ok_or("cargo metadata: deps")? {
            let kinds = dependency["dep_kinds"]
                .as_array()
                .ok_or("cargo metadata: dep_kinds")?;
            if kinds.iter().any(|kind| kind["kind"] != "dev") {
                pending.push(field(dependency, "pkg")?);
            }
        }
    }
    for package in metadata["packages"]
        .as_array()
        .ok_or("cargo metadata: packages")?
    {
        let id = field(package, "id")?;
        // Path packages live under the repository's rust/ directory and have
        // already been added to the project portion of the archive.
        if package["source"].is_null() || !needed.contains(id) {
            continue;
        }
        let name = format!("{}-{}", field(package, "name")?, field(package, "version")?);
        if let Some(previous) = sources.get(&name) {
            if previous.id != id {
                return Err(format!("Duas origens para a dependência Rust {name}").into());
            }
            continue;
        }
        let manifest = Path::new(field(package, "manifest_path")?);
        let directory = manifest
            .parent()
            .ok_or("Manifesto de crate inválido")?
            .to_owned();
        sources.insert(
            name,
            Source {
                id: id.into(),
                directory,
                license: package["license"].clone(),
            },
        );
    }
    Ok(())
}

pub(super) fn append(root: &Path, work: &Path, archive: &Path, manifests: &[&str]) -> Result<()> {
    let compiler = capture(Command::new("rustc").arg("-vV"))?;
    let host = compiler
        .lines()
        .find_map(|l| l.strip_prefix("host: "))
        .ok_or("Rust não informou a arquitetura de compilação")?;
    let mut sources = BTreeMap::new();
    for manifest in manifests {
        let output = capture(
            Command::new("cargo")
                .current_dir(root)
                .args([
                    "metadata",
                    "--format-version",
                    "1",
                    "--locked",
                    "--filter-platform",
                    host,
                    "--manifest-path",
                ])
                .arg(root.join(manifest)),
        )?;
        collect(&serde_json::from_str(&output)?, &mut sources)?;
    }
    // A versioned directory per crate also supports independent manifests
    // selecting different releases. Keep notices alongside each crate's code.
    let staging = tempfile::tempdir_in(work)?;
    let relative = "emwm-desktop/rust/dependency-sources";
    let destination = staging.path().join(relative);
    fs::create_dir_all(&destination)?;
    let mut index = Vec::new();
    for (name, source) in sources {
        run(Command::new("cp")
            .args(["-a", "--reflink=auto", "--"])
            .arg(&source.directory)
            .arg(destination.join(&name)))?;
        index.push(json!({"directory":name, "package":source.id,"license":source.license}));
    }
    fs::write(
        destination.join("sources.json"),
        serde_json::to_vec_pretty(&index)?,
    )?;
    run(Command::new("tar")
        .arg("--append")
        .arg("--file")
        .arg(archive)
        .args(["--exclude=.git", "--exclude=.cargo-ok", "-C"])
        .arg(staging.path())
        .arg(relative))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collects_normal_and_build_dependencies_once() -> Result<()> {
        let dep = |id: &str, kind: Value| json!({"pkg":id,"dep_kinds":[{"kind":kind}]});
        let node = |id: &str, deps: Vec<Value>| json!({"id":id,"deps":deps});
        let pkg = |name: &str| json!({"id":name,"name":name,"version":"1.2.0","manifest_path":format!("/registry/{name}-1.2.0/Cargo.toml"),"source":"registry+example","license":"MIT"});
        let graph = json!({"resolve":{"root":"app","nodes":[
            node("app",vec![dep("common",Value::Null),dep("macro",json!("build")),dep("test",json!("dev"))]),
            node("macro",vec![dep("common",Value::Null)]),node("common",vec![]),node("test",vec![]) ]},
            "packages":[{"id":"app","source":null},pkg("common"),pkg("macro"),pkg("test")]});
        let mut sources = BTreeMap::new();
        collect(&graph, &mut sources)?;
        collect(&graph, &mut sources)?;
        assert_eq!(
            sources.keys().collect::<Vec<_>>(),
            ["common-1.2.0", "macro-1.2.0"]
        );
        Ok(())
    }
}
