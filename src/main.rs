#[cfg(feature = "databaseflag")]
use ::{once_cell::sync::Lazy, surrealdb::engine::any, surrealdb::Surreal};

#[cfg(feature = "databaseflag")]
static DB: Lazy<Surreal<any::Any>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() {
    if cfg!(feature = "databaseflag") {
        #[cfg(feature = "databaseflag")]
        {
            createdatabase().await;
        }
    } else {
        println!("Ran without creating a database, try using one of the features:\ndatabase\nmemdatabase\nfiledatabase");
    }
    println!("Checking server function");
    let version = DB.version().await.unwrap();
    dbg!(version);
}

#[cfg(feature = "databaseflag")]
async fn createdatabase() {
    println!("Creating a database");
    connectdatabase().await;
    match DB.use_ns("some_namespace").use_db("some_database").await {
        Ok(val) => val,
        Err(dberror) => panic!("failed to use namespace or datebase: {dberror}"),
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature = "memdatabase")] {
        async fn connectdatabase() {         match DB.connect("mem://").await {
            Ok(val) => val,
            Err(dbconnecterror) => panic!("failed to connect to database: {dbconnecterror}"),
        };
        println!("Created an in memory database");
    }
    } else if #[cfg(feature = "filedatabase")] {
        async fn connectdatabase() {         match DB.connect("file://database.db").await {
            Ok(val) => val,
            Err(dbconnecterror) => panic!("failed to connect to database: {dbconnecterror}"),
        };
        println!("Created a file database"); }
    } else if #[cfg(feature = "database")]{

        async fn connectdatabase() {
            let remoteaddress = match std::env::var("SURREAL_BIND") {
                Ok(val) => {if !val.starts_with(|x :char| x.is_ascii_digit()) {val} else {
                    format!("ws://{val}")
                }},
                Err(_) => "ws://0.0.0.0:8000".to_string()
            };
            println!("Connecting to a database on address: {remoteaddress}");
            match DB.connect(remoteaddress).await {
            Ok(val) => val,
            Err(dbconnecterror) => panic!("failed to connect to database: {dbconnecterror}"),
        };
        println!("Created a remote database");
    }
    }
}
