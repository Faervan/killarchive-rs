use tokio_postgres::{Client, Error};

pub async fn schema_create(client: &Client) -> Result<(), Error> {
    println!("\nApplying the schema...\n");

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS alliances (
            id          varchar(25) NOT NULL primary key,
            name        varchar(30) NOT NULL,
            kills       integer NOT NULL,
            deaths      integer NOT NULL,
            assists     integer NOT NULL,
            allies      integer NOT NULL
        );
        CREATE INDEX ix_alliance_name ON alliances (name)
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS guilds (
            id          varchar(25) NOT NULL primary key,
            name        varchar(30) NOT NULL,
            alliance    varchar(25) references alliances(id),
            kills       integer NOT NULL,
            deaths      integer NOT NULL,
            assists     integer NOT NULL,
            allies      integer NOT NULL
        );
        CREATE INDEX ix_guild_name ON guilds (name)
    ").await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS players (
            id          varchar(25) NOT NULL primary key,
            name        varchar(30) NOT NULL,
            guild       varchar(25) references guilds(id),
            kills       integer NOT NULL,
            deaths      integer NOT NULL,
            assists     integer NOT NULL,
            allies      integer NOT NULL
        );
        CREATE INDEX ix_player_name ON players (name)
    ").await?;

    Ok(())
}

pub async fn schema_drop(client: &Client) -> Result<(), Error> {
    println!("\nDropping the schema...\n");

    client.batch_execute("DROP TABLE alliances CASCADE").await?;
    client.batch_execute("DROP TABLE guilds CASCADE").await?;
    client.batch_execute("DROP TABLE players").await?;

    Ok(())
}
