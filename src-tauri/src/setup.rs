use crate::models::notes::Note;
use crate::search::{self, TextIndexSearcher, TextIndexWriter};
use crate::utils::get_connection;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
use futures::prelude::*;
use futures::stream::FuturesUnordered;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tauri::{App, AppHandle, Manager};
use tokio::sync::oneshot::{self, Receiver};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub struct PoolWrapper {
    pub pool: SqlitePool,
}

pub struct WriterWrapper {
    pub writer: Arc<TextIndexWriter>,
}

pub struct SearcherWrapper {
    pub searcher: Arc<TextIndexSearcher>,
}

pub fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Setting up app");
    let path = make_path(app);

    let handle = app.handle();

    tauri::async_runtime::block_on(async_setup(path, handle));

    println!("Completed in {:.2?}", start.elapsed());
    Ok(())
}

async fn async_setup(path: PathBuf, app: &AppHandle) {
    let pool = get_pool(path.clone()).shared();

    let (searcher_sender, searcher) = oneshot::channel();
    let (writer_sender, writer) = oneshot::channel();
    let (needs_reindex_sender, needs_reindex) = oneshot::channel();

    let searcher = searcher.shared();
    let writer = writer.shared();

    let tasks = FuturesUnordered::new();

    let search_initializer = tokio::task::spawn(async move {
        search::initialize(&path, writer_sender, searcher_sender, needs_reindex_sender).await;
    });
    tasks.push(search_initializer);

    println!("Setting up app management");

    let handle = app.clone();
    let handled_writer = writer.clone();
    let tantivy_task = tokio::task::spawn(async move {
        let writer_wrapper = WriterWrapper {
            writer: handled_writer.await.expect("should create a valid writer"),
        };
        handle.manage(writer_wrapper);
        let searcher_wrapper = SearcherWrapper {
            searcher: searcher.await.expect("should be able to get a searcher"),
        };
        handle.manage(searcher_wrapper);
    });
    tasks.push(tantivy_task);

    let managed_pool = pool.clone();
    let handle = app.clone();
    let pool_task = tokio::task::spawn(async move {
        let pool_wrapper = PoolWrapper {
            pool: managed_pool.await.clone(),
        };
        handle.manage(pool_wrapper);
    });
    tasks.push(pool_task);

    let reindex_task = tokio::task::spawn(async move {
        reindex_if_needed(
            needs_reindex,
            pool.await,
            writer.await.expect("should have a valid writer"),
        )
        .await;
    });
    tasks.push(reindex_task);

    let _: Vec<_> = tasks.collect().await;

    println!("App setup complete");
}

fn make_path(app: &App) -> PathBuf {
    println!("Getting app data directory");
    let path = app
        .path()
        .app_data_dir()
        .expect("This should never be None");

    let path_str = path.clone().into_os_string().into_string().unwrap();
    println!("App data directory is {path_str}");
    println!("Ensuring {path_str} exists");
    create_dir_all(&path).expect("Should be able to create the directory");

    path
}

async fn reindex_if_needed(
    needs_reindex: Receiver<bool>,
    pool: Pool<ConnectionManager<SqliteConnection>>,
    tantivy_writer: Arc<TextIndexWriter>,
) {
    let mut conn = get_connection(pool);
    let all_notes = Note::get_all(&mut conn).expect("It should be able to get notes");

    if needs_reindex
        .await
        .expect("It should be told if it needs to reindex")
    {
        println!("Initializing tantivy index");
        let tasks = FuturesUnordered::new();
        for note in all_notes {
            let writer = tantivy_writer.clone();
            let task = tokio::task::spawn(async {
                let _ = search::write_note(note, writer);
            });
            tasks.push(task);
        }
        let _: Vec<_> = tasks.collect().await;
        println!("Tantivy index created");
    };
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

async fn get_pool(path: PathBuf) -> Pool<ConnectionManager<SqliteConnection>> {
    let db_path = path.join("data.db");
    let db_path = db_path.to_str().expect("Could not create DB path");
    println!("Initializing DB connection to {db_path}");

    let mut conn =
        SqliteConnection::establish(db_path).expect("Could not establish connection to database");

    println!("Running any pending migrations");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");

    println!("Creating DB connection manager");

    let manager = ConnectionManager::<SqliteConnection>::new(db_path);

    println!("Creating DB pool");

    Pool::builder()
        .build(manager)
        .expect("Could not create connection pool")
}
