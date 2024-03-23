use octocrab::{models::Repository, params};
use std::usize;

#[tokio::main]
async fn main() {
    let username = std::env::args().nth(1).expect("Provide a username");
    let language = std::env::args().nth(2).expect("No language provided");
    let count = std::env::args().nth(3).expect("No count provided");

    println!("Got username: {}", username);
    println!("Got language: {}", language);

    match count.parse::<usize>() {
        Ok(num) => {
            println!("Got the number: {}", num);
        }
        Err(_) => {
            println!("Failed to parse number from arg value: {}", count)
        }
    }

    let future = get_first_repo();
    let repo = future.await;

    match repo {
        Some(val) => {
            println!("Woo!: {}", val.full_name.unwrap());
        },
        None => {
            println!("Boo!");
        }
    }
}

async fn get_first_repo() -> Option<octocrab::models::Repository> {
    let result = octocrab::instance()
        .orgs("boomerang-io")
        .list_repos()
        // Optional Parameters
        .sort(params::repos::Sort::Pushed)
        .direction(params::Direction::Descending)
        // Send the request.
        .send()
        .await;

    match result {
        Ok(page) => {
            println!("Got results: {:?}", page.items.len());
            let first_repo: Repository = page.items.first().cloned().expect("WHAT");
            let name_option = &first_repo.full_name;
            match name_option {
                Some(val) => {
                    println!("Got repo: {:?}", val);
                },
                None => {
                    println!("Repo doesn't have a name somehow");
                }
            }

            return Some(first_repo);
        }
        Err(_) => {
            println!("Something went wrong");
            return None;
        }
    }
}
