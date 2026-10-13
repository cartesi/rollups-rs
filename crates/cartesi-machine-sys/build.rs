// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)
use std::{
    env,
    path::{Path, PathBuf},
};

const C_API_HEADER: &str = "cm.h";
const C_API_VERSION_HEADER: &str = "cm-version.h";

// The emulator release line these bindings target. `cm.h` is only stable
// within a minor release, so any other version is refused.
const SUPPORTED_VERSION: (u32, u32) = (0, 21);

// Installation prefixes searched, in order, when `LIBCARTESI_PATH` is unset:
// Homebrew (Apple silicon), the conventional local prefix (also Homebrew on
// Intel), MacPorts, and the system prefix used by the release `.deb`.
const SEARCH_PREFIXES: [&str; 4] = ["/opt/homebrew", "/usr/local", "/opt/local", "/usr"];

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let (lib_dir, include_path) = locate_libcartesi();

    link_external(&lib_dir, &out_path);

    // Static link. Release packages may build libcartesi with slirp;
    // `link_external` adds that dependency when the archive references it.
    if cfg!(feature = "remote_machine") {
        println!("cargo:rustc-link-lib=static=cartesi_jsonrpc");
    } else {
        println!("cargo:rustc-link-lib=static=cartesi");
    }

    // OpenMP linker configuration (cross-platform)
    if cfg!(target_os = "macos") {
        // macOS: Try Homebrew first, then MacPorts
        let homebrew_libomp = PathBuf::from("/opt/homebrew/opt/libomp");
        if homebrew_libomp.exists() {
            println!("cargo:rustc-link-search={}/lib", homebrew_libomp.display());
            println!("cargo:rustc-link-lib=omp");
        } else {
            let macports_libomp = PathBuf::from("/opt/local/lib/libomp");
            if macports_libomp.exists() {
                println!("cargo:rustc-link-search=/opt/local/lib/libomp");
                println!("cargo:rustc-link-lib=gomp");
            } else {
                // Fallback: let system linker find it
                println!("cargo:rustc-link-lib=omp");
            }
        }
    } else {
        // Linux and other Unix-like systems: libgomp comes with GCC
        println!("cargo:rustc-link-lib=gomp");
    }

    let api_header = include_path.join(C_API_HEADER);
    let api_version_header = include_path.join(C_API_VERSION_HEADER);
    assert!(
        api_header.is_file(),
        "Cartesi Machine C API header not found at `{}`; set `INCLUDECARTESI_PATH` to the directory containing `{C_API_HEADER}`",
        api_header.display()
    );
    assert!(
        api_version_header.is_file(),
        "Cartesi Machine version header not found at `{}`; `{C_API_HEADER}` includes `{C_API_VERSION_HEADER}`",
        api_version_header.display()
    );
    println!("cargo:rerun-if-changed={}", api_header.display());
    println!("cargo:rerun-if-changed={}", api_version_header.display());
    check_version(&api_version_header);

    let machine_bindings = bindgen::Builder::default()
        .header(api_header.to_str().unwrap())
        .allowlist_item("^cm_.*")
        .allowlist_item("^CM_.*")
        .merge_extern_blocks(true)
        .prepend_enum_name(false)
        .translate_enum_integer_types(true)
        .generate()
        .expect("Unable to generate machine bindings");

    machine_bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write machine bindings");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=LIBCARTESI_PATH");
    println!("cargo::rerun-if-env-changed=INCLUDECARTESI_PATH");
}

/// Returns the directories holding the static archives and the C API headers.
///
/// `LIBCARTESI_PATH` selects the archive directory explicitly. Otherwise the
/// first of `SEARCH_PREFIXES` that holds both the archives and the headers
/// wins. `INCLUDECARTESI_PATH` overrides the header directory either way.
fn locate_libcartesi() -> (PathBuf, PathBuf) {
    let include_override = env::var_os("INCLUDECARTESI_PATH").map(|path| {
        let path = PathBuf::from(path);
        assert!(
            path.is_absolute(),
            "INCLUDECARTESI_PATH must be an absolute directory, found `{}`",
            path.display()
        );
        path
    });

    if let Some(lib_dir) = env::var_os("LIBCARTESI_PATH").map(PathBuf::from) {
        assert!(
            lib_dir.is_absolute(),
            "LIBCARTESI_PATH must be an absolute directory, found `{}`",
            lib_dir.display()
        );
        let include_path = include_override.unwrap_or_else(|| inferred_include_path(&lib_dir));
        return (lib_dir, include_path);
    }

    for prefix in SEARCH_PREFIXES {
        let prefix = Path::new(prefix);
        let lib_dir = prefix.join("lib");
        let include_path = include_override
            .clone()
            .unwrap_or_else(|| prefix.join("include/cartesi-machine"));
        if required_archives().all(|archive| lib_dir.join(archive).is_file())
            && include_path.join(C_API_HEADER).is_file()
        {
            return (lib_dir, include_path);
        }
    }

    panic!(
        "Cartesi Machine {}.{} not found in {}.\n\
         Install the emulator (https://github.com/cartesi/machine-emulator/releases), \
         or set `LIBCARTESI_PATH` to the absolute directory containing `{}` \
         (and `INCLUDECARTESI_PATH` to the directory containing `{C_API_HEADER}`).",
        SUPPORTED_VERSION.0,
        SUPPORTED_VERSION.1,
        SEARCH_PREFIXES.join(", "),
        required_archives().collect::<Vec<_>>().join("` and `"),
    );
}

fn required_archives() -> impl Iterator<Item = &'static str> {
    let jsonrpc = cfg!(feature = "remote_machine").then_some("libcartesi_jsonrpc.a");
    std::iter::once("libcartesi.a").chain(jsonrpc)
}

fn inferred_include_path(lib_dir: &Path) -> PathBuf {
    lib_dir
        .parent()
        .unwrap_or_else(|| {
            panic!(
                "cannot infer the Cartesi Machine include directory from `{}`; set `INCLUDECARTESI_PATH`",
                lib_dir.display()
            )
        })
        .join("include/cartesi-machine")
}

/// Refuses headers from an emulator release other than `SUPPORTED_VERSION`.
fn check_version(version_header: &Path) {
    let contents = std::fs::read_to_string(version_header)
        .unwrap_or_else(|e| panic!("failed to read `{}`: {e}", version_header.display()));
    let define = |name: &str| -> u32 {
        contents
            .lines()
            .find_map(|line| {
                let mut words = line.split_whitespace();
                (words.next() == Some("#define") && words.next() == Some(name))
                    .then(|| words.next()?.parse().ok())
                    .flatten()
            })
            .unwrap_or_else(|| panic!("`{name}` not found in `{}`", version_header.display()))
    };
    let found = (define("CM_VERSION_MAJOR"), define("CM_VERSION_MINOR"));
    assert_eq!(
        found,
        SUPPORTED_VERSION,
        "`{}` is Cartesi Machine {}.{}, but these bindings require {}.{}",
        version_header.display(),
        found.0,
        found.1,
        SUPPORTED_VERSION.0,
        SUPPORTED_VERSION.1,
    );
}

// Stage the external static archives into OUT_DIR and search only there.
// Searching the provider's lib dir directly is not safe: it usually also
// contains libcartesi dylibs, and ld64 prefers a dylib over an archive
// even under rustc's `static=` modifier, producing binaries that need an
// rpath into the provider's tree at runtime.
fn link_external(libdir: &Path, out_path: &Path) {
    let cartesi = libdir.join("libcartesi.a");
    println!("cargo:rerun-if-changed={}", cartesi.display());
    stage_archive(&cartesi, out_path);
    let mut slirp = references_slirp(&cartesi);

    if cfg!(feature = "remote_machine") {
        let jsonrpc = libdir.join("libcartesi_jsonrpc.a");
        println!("cargo:rerun-if-changed={}", jsonrpc.display());
        stage_archive(&jsonrpc, out_path);
        slirp |= references_slirp(&jsonrpc);
    }

    println!("cargo:rustc-link-search={}", out_path.display());
    if slirp {
        link_slirp();
    }
}

// An archive built with slirp names slirp's symbols; a `slirp=no` one (Nix's)
// does not, and linking libslirp anyway requires it at build time and loads
// it at run time for nothing.
fn references_slirp(archive: &Path) -> bool {
    let bytes = std::fs::read(archive)
        .unwrap_or_else(|e| panic!("failed to read `{}`: {e}", archive.display()));
    let symbol = b"slirp_new";
    bytes.windows(symbol.len()).any(|window| window == symbol)
}

// ld64 searches neither Homebrew's nor MacPorts' library directory by
// default. Homebrew's keg holds only libslirp; MacPorts' directory is shared,
// but it is searched after OUT_DIR, which already holds the staged archives.
fn link_slirp() {
    if cfg!(target_os = "macos") {
        for dir in ["/opt/homebrew/opt/libslirp/lib", "/opt/local/lib"] {
            if Path::new(dir).join("libslirp.dylib").exists() {
                println!("cargo:rustc-link-search={dir}");
                break;
            }
        }
    }
    println!("cargo:rustc-link-lib=slirp");
}

fn stage_archive(archive: &Path, out_path: &Path) {
    let staged = out_path.join(archive.file_name().unwrap());
    // fs::copy preserves the source mode; a nix-store source stages a
    // read-only copy that the next build-script run cannot overwrite.
    if staged.exists() {
        std::fs::remove_file(&staged)
            .unwrap_or_else(|e| panic!("failed to unstage `{}`: {e}", staged.display()));
    }
    std::fs::copy(archive, &staged)
        .unwrap_or_else(|e| panic!("failed to copy `{}` to OUT_DIR: {e}", archive.display()));
}
