use error::Error;

use cornucopia::cornucopia_gen;
use deadpool_postgres::Client;

async fn setup() -> Result<Client, cornucopia::error::Error> {
    use cornucopia::run_migrations::run_migrations;
    use cornucopia::{container, pool::cli_pool};

    container::setup()?;
    let pool = cli_pool()?;
    let client = pool.get().await?;
    run_migrations(&client, "tests/migrations").await?;

    Ok(client)
}

async fn teardown() -> Result<(), cornucopia::error::Error> {
    use cornucopia::container;
    container::cleanup()?;
    Ok(())
}

async fn integration() -> Result<(), Error> {
    let client = setup().await?;

    authors_test(&client).await?;
    books_test(&client).await?;
    books_from_author_id_test(&client).await?;
    author_name_by_id_test(&client).await?;
    author_name_by_id_opt_test(&client).await?;
    author_name_starting_with_test(&client).await?;
    insert_books_test(&client).await?;

    teardown().await?;

    Ok(())
}

#[tokio::test]

async fn integration_test() {
    if let Err(e) = integration().await {
        println!("{}", e);
        let _ = teardown().await;
    }
}

/// This test monitors the behaviour of queries
/// that don't return anything. In this case,
/// the quantifier should be ignored.
async fn insert_books_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::module_1::{
        insert_book_one, insert_book_zero_or_more, insert_book_zero_or_one,
    };

    let i1 = insert_book_one(&client).await?;
    let i2 = insert_book_zero_or_more(&client).await?;
    let i3 = insert_book_zero_or_one(&client).await?;

    if () != i1 {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", ()),
            actual: format!("{:?}", i1),
        });
    };
    if () != i2 {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", ()),
            actual: format!("{:?}", i2),
        });
    };
    if () != i3 {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", ()),
            actual: format!("{:?}", i3),
        });
    };
    Ok(())
}

async fn authors_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::module_2::authors;

    let expected = vec![
        (
            1,
            String::from("Agatha Christie"),
            String::from("United Kingdom"),
        ),
        (
            2,
            String::from("John Ronald Reuel Tolkien"),
            String::from("United Kingdom"),
        ),
    ];
    let actual = authors(&client).await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };

    Ok(())
}

async fn books_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::module_2::{books, Books};

    let expected = vec![
        Books {
            title: String::from("The Silmarillion"),
        },
        Books {
            title: String::from("The Hobbit"),
        },
        Books {
            title: String::from("Murder on the Orient Express"),
        },
        Books {
            title: String::from("Death on the Nile"),
        },
    ];

    let actual = books(&client).await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };

    Ok(())
}

async fn books_from_author_id_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::module_2::books_from_author_id;

    let expected = vec![
        String::from("Death on the Nile"),
        String::from("Murder on the Orient Express"),
    ];
    let actual = books_from_author_id(&client, &0).await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };

    Ok(())
}

async fn author_name_by_id_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::module_2::author_name_by_id;
    let expected = String::from("Agatha Christie");
    let actual = author_name_by_id(client, &1).await?;

    if expected != actual {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };
    Ok(())
}

async fn author_name_by_id_opt_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::module_2::author_name_by_id_opt;
    let expected = None;
    let actual = author_name_by_id_opt(client, &-1).await?;

    if expected != actual {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    };
    Ok(())
}

async fn author_name_starting_with_test(client: &Client) -> Result<(), Error> {
    use cornucopia_gen::module_2::author_name_starting_with;
    let expected = vec![
        (
            2,
            String::from("John Ronald Reuel Tolkien"),
            3,
            String::from("The Hobbit"),
        ),
        (
            2,
            String::from("John Ronald Reuel Tolkien"),
            4,
            String::from("The Silmarillion"),
        ),
    ];
    let actual = author_name_starting_with(client, "Jo").await?;

    if !actual.iter().all(|item| expected.contains(item)) {
        return Err(Error::IntegrationError {
            expected: format!("{:?}", expected),
            actual: format!("{:?}", actual),
        });
    }

    Ok(())
}

mod error {
    use cornucopia::error::Error as CornucopiaError;
    use thiserror::Error as ThisError;
    use tokio_postgres::Error as DbError;
    #[derive(Debug, ThisError)]
    #[error("error occured during integration testing")]
    pub enum Error {
        #[error("expected {expected}, got {actual}")]
        IntegrationError {
            expected: String,
            actual: String,
        },
        DbError(#[from] DbError),
        CornucopiaError(#[from] CornucopiaError),
    }
}