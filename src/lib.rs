mod tests;

use std::process::{Command, Stdio};

use failure::{format_err, Error};
use postgres::{params, Connection, TlsMode};

#[derive(Clone)]
pub struct PostgresContainer {
    name: String,
    port: u16,
}

impl Drop for PostgresContainer {
    fn drop(&mut self) {
        let _ = Command::new("docker")
            .arg("rm")
            .arg("-f")
            .arg(self.name.as_str())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();
    }
}

impl PostgresContainer {
    pub fn new(name: &str, port: u16) -> Result<PostgresContainer, Error> {
        Command::new("docker")
            .arg("run")
            .arg("-p")
            .arg(format!("127.0.0.1:{}:5432", port).as_str())
            .arg("--name")
            .arg(name)
            .arg("postgres:latest")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(PostgresContainer {
            name: String::from(name),
            port: port,
        })
    }
    pub fn connect(&self) -> Result<Connection, Error> {
        Ok(Connection::connect(
            params::Builder::new()
                .port(self.port)
                .user("postgres", None)
                .build(params::Host::Tcp(String::from("127.0.0.1"))),
            TlsMode::None,
        )?)
    }

    pub fn heavy_connect(
        &self,
        attempts: usize,
        retry: std::time::Duration,
    ) -> Result<Connection, Error> {
        for _ in 0..attempts {
            if let Ok(c) = self.connect() {
                return Ok(c);
            } else {
                std::thread::sleep(retry);
            }
        }
        Err(format_err!(
            "Could not connect to database on port {} after {} attempts",
            self.port,
            attempts
        ))
    }
}
