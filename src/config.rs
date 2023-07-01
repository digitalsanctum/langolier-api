/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
///
/// See `.env.sample` in the repository root for details.
#[derive(clap::Parser, Debug, Clone, PartialEq)]
pub struct Config {
    /// The connection URL for the Postgres database.
    #[clap(long, env )]
    pub database_url: String,

    #[clap(long, env)]
    pub port: Option<u16>,

    /// The connection URL for the NATS server.
    #[clap(long, env, default_value = "nats://localhost:4222")]
    pub nats_url: String,
}
