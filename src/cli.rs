#[derive(Clone)]
pub struct Args {
    pub download_path: String,
    pub ip: String,
    pub port: u16,
    pub workers: u8,
    pub v_lvl: u8,
    pub dbhost: String,
    pub dbuser: String,
    pub dbpwd: String,
    pub dbdb: String,
    pub jwt_secret: String,
}

impl Args {
    
    pub fn get_args() -> Self {
        let matches = clap::App::new("soundloop")
            .version("1.0")
            .author("lol <lol@lol>")
            .about("Soundloop Server")
            .arg(clap::Arg::new("ip")
                .short('i')
                .long("ip")
                .value_name("IP")
                .about("Sets the bind ip of the server")
                .takes_value(true))
            .arg(clap::Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .about("Sets the bind port of the server")
                .takes_value(true))
            .arg(clap::Arg::new("workers")
                .short('w')
                .long("workers")
                .value_name("WORKERS")
                .about("Number of download worker threads to run in parallel.")
                .takes_value(true))
            .arg(clap::Arg::new("dir")
                .short('d')
                .long("dir")
                .value_name("DOWNLOAD_DIR")
                .about("Set the download directory.")
                .takes_value(true))
            .arg(clap::Arg::new("dbhost")
                .long("dbhost")
                .value_name("DBHOST")
                .about("Set the host of the database.")
                .takes_value(true))
            .arg(clap::Arg::new("dbuser")
                .long("dbuser")
                .value_name("DBUSER")
                .about("Set the username of the database.")
                .takes_value(true))
            .arg(clap::Arg::new("dbpwd")
                .long("dbpwd")
                .value_name("DBPWD")
                .about("Set the password of the database.")
                .takes_value(true))
            .arg(clap::Arg::new("dbdb")
                .long("dbdb")
                .value_name("DBDB")
                .about("Set the database name for Soundloop.")
                .takes_value(true))
            .arg(clap::Arg::new("jwt")
                .short('j')
                .long("jwt")
                .value_name("JWT")
                .about("Set the JWT secret key.")
                .required(true)
                .takes_value(true))
            .arg(clap::Arg::new("verbose")
                .short('v')
                .about("Sets the level of verbosity")
                .multiple(true))
            .get_matches();
        let download_path = matches.value_of("dir").unwrap_or("downloads").to_owned();
        let ip = matches.value_of("ip").unwrap_or("0.0.0.0").to_owned();
        let port = matches.value_of("port").unwrap_or("8000").parse::<u16>().unwrap_or(8000);
        let workers = matches.value_of("workers").unwrap_or("5").parse::<u8>().unwrap_or(5);
        let v_lvl = matches.value_of("verbose").unwrap_or("0").parse::<u8>().unwrap_or(0);
        let dbhost = matches.value_of("dbhost").unwrap_or("couchbase://localhost").to_owned();
        let dbuser = matches.value_of("dbuser").unwrap_or("").to_owned();
        let dbpwd = matches.value_of("dbpwd").unwrap_or("").to_owned();
        let dbdb = matches.value_of("dbdb").unwrap_or("Soundloop").to_owned();
        let jwt_secret = matches.value_of("jwt").unwrap_or("").to_owned();
        Self {
            download_path,
            ip,
            port,
            workers,
            v_lvl,
            dbhost,
            dbuser,
            dbpwd,
            dbdb,
            jwt_secret,
        }
    }

}
