use std::path::PathBuf;
use std::{io::Cursor, println};

use rstest::rstest;
use yansi::Paint as _;

use crate::command::Args;
use crate::language::Language;
use crate::tests::helpers::test_run::TestRun;
use crate::wipe::Wipe;
use crate::writer::{SPACING_FILES, SPACING_SIZE};

#[rstest]
#[case(Language::Node, false)]
#[case(Language::Node, true)]
#[case(Language::Rust, false)]
#[case(Language::Rust, true)]
#[case(Language::Terraform, false)]
#[case(Language::Terraform, true)]
fn run_with_hits(#[case] language: Language, #[case] wipe: bool) {
    for directory in language.dirs() {
        let test_run = TestRun::new(&language, 3, 0);

        let args = Args {
            language,
            wipe,
            ignores: Vec::new(),
            path: PathBuf::from(&test_run),
        };

        let mut buff = Cursor::new(Vec::new());
        Wipe::new(&mut buff).run(&args).unwrap();

        let output = std::str::from_utf8(buff.get_ref()).unwrap();
        println!("{output}");

        // header
        let expected = format!("{}", "[DRY RUN]".green().bold());
        assert_eq!(output.contains(&expected), !wipe);

        let expected = format!("{}", "[WIPING]".red().bold());
        assert_eq!(output.contains(&expected), wipe);

        let expected = format!(r#""{}""#, directory.cyan());
        assert!(output.contains(&expected));

        // body
        // hits should be listed and wiped if wipe is true
        for path in &test_run.hits {
            let expected = String::from(path.to_str().unwrap());
            assert!(output.contains(&expected));
            assert_eq!(path.exists(), !wipe);
        }

        // misses should not be listed and not wiped
        for path in &test_run.misses {
            let expected = String::from(path.to_str().unwrap());
            assert!(!output.contains(&expected));
            assert!(path.exists());
        }

        // summary should be displayed
        let expected = format!("{:>files$}", "Files #".cyan(), files = SPACING_FILES);
        let output = output.replacen(&expected, "", 1);
        assert!(output.contains(&expected));

        let expected = format!("{:>size$}", "Size (MB)".cyan(), size = SPACING_SIZE);
        let output = output.replacen(&expected, "", 1);

        let expected = format!("{:>size$}", "Size".cyan(), size = SPACING_SIZE);
        assert!(output.contains(&expected));

        let expected = format!("{}", test_run.path.display().cyan());
        let output = &output.replacen(&expected, "", 1);
        assert!(output.contains(&expected));

        let expected = format!("{}", "Ignored".yellow());
        assert!(!output.contains(&expected));

        // footer
        if wipe {
            let expected = format!("{}", "All clear!".green());
            assert!(output.contains(&expected));
        } else {
            let expected = format!(
                "Run {} to wipe all folders found. {}",
                format!("cargo wipe {} -w", args.language).red(),
                "USE WITH CAUTION!".red()
            );
            assert!(output.contains(&expected));
        }
    }
}

#[rstest]
#[case(Language::Node, false)]
#[case(Language::Node, true)]
#[case(Language::Rust, false)]
#[case(Language::Rust, true)]
#[case(Language::Terraform, false)]
#[case(Language::Terraform, true)]
fn run_no_hits(#[case] language: Language, #[case] wipe: bool) {
    let test_run = TestRun::new(&language, 0, 0);

    let args = Args {
        wipe,
        path: PathBuf::from(&test_run),
        language,
        ignores: Vec::new(),
    };

    let mut buff = Cursor::new(Vec::new());
    Wipe::new(&mut buff).run(&args).unwrap();

    let output = std::str::from_utf8(buff.get_ref()).unwrap();
    println!("{output}");

    // body
    let expected = format!("{}", "Files #".cyan());
    assert!(!output.contains(&expected));

    let expected = format!("{}", "Size".cyan());
    assert!(!output.contains(&expected));

    let expected = format!("{}", test_run.path.display().cyan());
    let output = &output.replacen(&expected, "", 1);
    assert!(!output.contains(&expected));

    // summary should not be displayed
    let expected = format!("{:>files$}", "Files #".cyan(), files = SPACING_FILES);
    let output = output.replacen(&expected, "", 1);
    assert!(!output.contains(&expected));

    let expected = format!("{:>size$}", "Size (MB)".cyan(), size = SPACING_SIZE);
    let output = output.replacen(&expected, "", 1);

    let expected = format!("{:>size$}", "Size".cyan(), size = SPACING_SIZE);
    assert!(!output.contains(&expected));

    let expected = format!("{}", test_run.path.display().cyan());
    let output = &output.replacen(&expected, "", 1);
    assert!(!output.contains(&expected));

    // footer
    let expected = format!("{}", "Nothing found!".green());
    assert!(output.contains(&expected));
}

#[rstest]
#[case(Language::Node, false)]
#[case(Language::Node, true)]
#[case(Language::Rust, false)]
#[case(Language::Rust, true)]
#[case(Language::Terraform, false)]
#[case(Language::Terraform, true)]
fn run_with_ignores(#[case] language: Language, #[case] wipe: bool) {
    let test_run = TestRun::new(&language, 3, 3);

    let args = Args {
        wipe,
        path: PathBuf::from(&test_run),
        language,
        ignores: test_run.ignores.clone(),
    };

    let mut buff = Cursor::new(Vec::new());
    Wipe::new(&mut buff).run(&args).unwrap();

    let output = std::str::from_utf8(buff.get_ref()).unwrap();
    let lines = output.lines();
    println!("{output}");

    // body
    // hits should be listed and wiped if wipe is true
    for path in &test_run.hits {
        let expected = String::from(path.to_str().unwrap());
        let mut lines = lines.clone();
        let line = lines.find(|l| l.contains(&expected));

        assert!(line.is_some());
        let line = line.unwrap();

        assert!(line.contains(&expected));
        assert!(!line.contains("[Ignored]"));
        assert_eq!(path.exists(), !wipe);
    }

    // ignores should be listed and not wiped if wipe is true
    for path in &test_run.ignores {
        let expected = String::from(path.to_str().unwrap());
        let mut lines = lines.clone();
        let line = lines.find(|l| l.contains(&expected));

        assert!(line.is_some());
        let line = line.unwrap();

        assert!(line.contains(&expected));
        assert!(line.contains("[Ignored]"));
        assert!(path.exists());
    }

    // misses should not be listed and not wiped
    for path in &test_run.misses {
        let expected = String::from(path.to_str().unwrap());
        let mut lines = lines.clone();
        let line = lines.find(|l| l.contains(&expected));

        assert!(line.is_none());
        assert!(path.exists());
    }

    // summary should be displayed
    let expected = format!("{}", "Ignored".yellow());
    assert!(output.contains(&expected));
}
