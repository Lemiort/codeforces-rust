use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const API_URL: &str = "https://codeforces.com/api/problemset.problems";
const TEMPLATE_PATH: &str = "templates/solution.rs";

#[derive(Debug, serde::Deserialize)]
struct ApiResponse {
    status: String,
    result: Option<ApiResult>,
}

#[derive(Debug, serde::Deserialize)]
struct ApiResult {
    problems: Vec<Problem>,
}

#[derive(Debug, serde::Deserialize)]
struct Problem {
    #[serde(rename = "contestId")]
    contest_id: Option<u32>,
    index: String,
    name: String,
    rating: Option<u32>,
    tags: Vec<String>,
}

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run --bin new_solution -- <contest_id><problem_index>");
        eprintln!("Example: cargo run --bin new_solution -- 808D");
        std::process::exit(2);
    });

    if let Err(error) = generate_solution(&argument) {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn generate_solution(argument: &str) -> Result<PathBuf, String> {
    let (contest_id, problem_index) = parse_problem_id(argument)?;
    let problem = fetch_problem(contest_id, &problem_index)?;
    let slug = slugify(&problem.name);
    let file_name = format!("s{}{}_{}.rs", contest_id, problem.index, slug);
    let destination = Path::new("src/bin").join(file_name);

    if destination.exists() {
        return Err(format!(
            "solution already exists: {}",
            destination.display()
        ));
    }

    let template = fs::read_to_string(TEMPLATE_PATH)
        .map_err(|error| format!("could not read {TEMPLATE_PATH}: {error}"))?;
    let source = render_template(&template, &problem, contest_id);

    fs::write(&destination, source)
        .map_err(|error| format!("could not write {}: {error}", destination.display()))?;

    println!("Created {}", destination.display());
    println!("Problem: {}", problem.name);
    println!(
        "Link: https://codeforces.com/problemset/problem/{contest_id}/{}",
        problem.index
    );
    Ok(destination)
}

fn parse_problem_id(argument: &str) -> Result<(u32, String), String> {
    let normalized = argument.trim().trim_end_matches('/');
    let parts: Vec<&str> = normalized.split('/').collect();
    let compact = if let Some(problem_position) = parts.iter().position(|part| *part == "problem") {
        if parts.len() <= problem_position + 2 {
            return Err("problem URL must include a contest ID and problem index".to_owned());
        }
        format!(
            "{}{}",
            parts[problem_position + 1],
            parts[problem_position + 2]
        )
    } else {
        normalized.to_owned()
    };

    let split_at = compact
        .find(|character: char| character.is_ascii_alphabetic())
        .ok_or_else(|| "expected an ID like 808D or a Codeforces problem URL".to_owned())?;
    let (contest, index) = compact.split_at(split_at);
    let contest_id = contest
        .parse::<u32>()
        .map_err(|_| format!("invalid contest ID: {contest}"))?;

    if index.is_empty()
        || !index
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
    {
        return Err(format!("invalid problem index: {index}"));
    }

    Ok((contest_id, index.to_ascii_uppercase()))
}

fn fetch_problem(contest_id: u32, problem_index: &str) -> Result<Problem, String> {
    let response = reqwest::blocking::get(API_URL)
        .map_err(|error| format!("could not fetch Codeforces API: {error}"))?;
    let api: ApiResponse = response
        .json()
        .map_err(|error| format!("could not parse Codeforces API response: {error}"))?;

    if api.status != "OK" {
        return Err("Codeforces API returned an error".to_owned());
    }

    api.result
        .ok_or_else(|| "Codeforces API response has no result".to_owned())?
        .problems
        .into_iter()
        .find(|problem| {
            problem.contest_id == Some(contest_id)
                && problem.index.eq_ignore_ascii_case(problem_index)
        })
        .ok_or_else(|| format!("problem {contest_id}{problem_index} was not found"))
}

fn slugify(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_was_separator = false;

    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !previous_was_separator && !slug.is_empty() {
            slug.push('_');
            previous_was_separator = true;
        }
    }

    slug.trim_end_matches('_').to_owned()
}

fn render_template(template: &str, problem: &Problem, contest_id: u32) -> String {
    let rating = problem
        .rating
        .map(|value| value.to_string())
        .unwrap_or_else(|| "unrated".to_owned());
    let tags = if problem.tags.is_empty() {
        "none".to_owned()
    } else {
        problem.tags.join(", ")
    };
    let link = format!(
        "https://codeforces.com/problemset/problem/{contest_id}/{}",
        problem.index
    );

    let header = format!(
        "// {}{}: {}\n// Problem: {}\n// Rating: {}\n// Tags: {}\n//\n#![allow(dead_code)]",
        contest_id, problem.index, problem.name, link, rating, tags
    );

    template.replacen("#![allow(dead_code)]", &header, 1)
}

#[cfg(test)]
mod tests {
    use super::{parse_problem_id, slugify};

    #[test]
    fn parses_compact_problem_ids() {
        assert_eq!(parse_problem_id("808D").unwrap(), (808, "D".to_owned()));
        assert_eq!(parse_problem_id("808d").unwrap(), (808, "D".to_owned()));
    }

    #[test]
    fn parses_problem_urls() {
        assert_eq!(
            parse_problem_id("https://codeforces.com/problemset/problem/808/D").unwrap(),
            (808, "D".to_owned())
        );
    }

    #[test]
    fn creates_readable_slugs() {
        assert_eq!(slugify("Array Division"), "array_division");
        assert_eq!(slugify("A+B? (Trial Problem)"), "a_b_trial_problem");
    }
}
