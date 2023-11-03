use std::io::Write;

use analyzer::analyze;
use image;
use pretty_assertions;
use rayon::prelude::*;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct Event<'a, T: Serialize> {
    name: &'a str,
    body: T,
}

#[test]
fn inputs_match_outputs() {
    let input_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/inputs");
    let paths = std::fs::read_dir(input_path).expect("Couldn't open inputs/ dir");
    let paths: Vec<_> = paths.filter_map(|p| p.ok()).map(|p| p.path()).collect();

    eprintln!("testing {} files", paths.len());

    let results: Vec<_> = paths
        .par_iter()
        .filter(|path| {
            let Ok(input_image) = image::open(&path) else {
            panic!("couldn't open input_image at {path:?}");
        };

            let result = analyze(&input_image);

            let actual = match result {
                Some(screen) => {
                    let event = Event {
                        name: screen.event_type(),
                        body: screen,
                    };
                    serde_json::to_string_pretty(&event).unwrap()
                }
                None => {
                    let event: Event<Option<String>> = Event {
                        name: "null screen".into(),
                        body: None,
                    };

                    serde_json::to_string_pretty(&event).unwrap()
                }
            };

            let expectation_path = path
                .to_str()
                .expect("not a valid path name")
                .to_string()
                .replace("/inputs/", "/outputs/")
                .replace(".jpg", ".json");

            // NOTE: uncomment the 3 lines below to update the expectations
            // use std::io::Write;
            // let mut expectation_file = std::fs::File::create(&expectation_path).expect("couldn't open expectation file");
            // write!(expectation_file, "{actual}").unwrap();

            let expectation =
                std::fs::read_to_string(&expectation_path).expect("no expectation found");

            if expectation != actual {
                eprintln!(
                    "output from {path:?} differs from expectation at {expectation_path}\n\n{}",
                    pretty_assertions::StrComparison::new(&expectation, &actual)
                );

                return false;
            }

            return true;
        })
        .collect();

    // we filter out all invalid results so if these don't match, we've failed
    assert_eq!(paths.len(), results.len());

    // NOTE: uncomment the below line to rebuild the review page
    // create_review_file();
}

// we only use this sometimes
#[allow(dead_code)]
fn create_review_file() {
    let input_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/inputs");
    let template_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/template.html");
    let review_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/review.html");
    let paths = std::fs::read_dir(input_path).expect("Couldn't open inputs/ dir");
    let paths: Vec<_> = paths.filter_map(|p| p.ok()).map(|p| p.path()).collect();

    let rows = paths.par_iter().map(|path| {
        let path_str = path.to_str().expect("not a valid path name");
        let expectation_path = path_str.to_string().replace("/inputs/", "/outputs/").replace(".jpg", ".json");
        let expectation = std::fs::read_to_string(expectation_path).expect("no expectation found");

        format!("<tr><td colspan=\"3\"><b>{path_str}</b></td></tr><tr><td><img src=\"{path_str}\" /></td><td><pre>{expectation}</pre></td><td><input type=\"checkbox\" /></td></tr>")
    }).collect::<Vec<_>>();

    let contents = rows.join("\n");

    let template = std::fs::read_to_string(template_path).expect("couldn't find template");

    let review_page = template.replace("{/*content*/}", &contents);

    write!(
        std::fs::File::create(review_path).expect("couldn't open review file for writing"),
        "{review_page}"
    )
    .unwrap();
}
