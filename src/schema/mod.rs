use tokio_postgres::{Client, Error};

pub async fn schema_create(client: &Client) -> Result<(), Error> {
    println!("\nApplying the schema...\n");

    let meta = "
        id                  varchar(25) NOT NULL primary key,
        name                varchar(30) NOT NULL,
    ";

    let count = "
        kills               integer NOT NULL,
        deaths              integer NOT NULL,
        assists             integer NOT NULL,
        allies              integer NOT NULL,
        winrate             smallint,
    ";

    let fame = "
        kill_fame           bigint NOT NULL,
        death_fame          bigint NOT NULL,
        fame_ratio          smallint,
    ";

    let time = "
        registered_since    timestamp NOT NULL DEFAULT NOW(),
        updated_at          timestamp NOT NULL DEFAULT NOW()
    ";

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS cached_events (
            id                  integer NOT NULL primary key,
            timestamp           timestamp NOT NULL DEFAULT NOW()
        )
    ").await?;

    client.batch_execute(&format!("
        CREATE TABLE IF NOT EXISTS alliances (
            {meta}
            {count}
            {fame}
            {time}
        );
        CREATE INDEX ix_alliance_name ON alliances (name)
    ")).await?;

    client.batch_execute(&format!("
        CREATE TABLE IF NOT EXISTS guilds (
            {meta}
            alliance            varchar(25) references alliances(id),
            {count}
            {fame}
            {time}
        );
        CREATE INDEX ix_guild_name ON guilds (name)
    ")).await?;

    client.batch_execute(&format!("
        CREATE TABLE IF NOT EXISTS players (
            {meta}
            guild               varchar(25) references guilds(id),
            {count}
            {fame}
            {time}
        );
        CREATE INDEX ix_player_name ON players (name)
    ")).await?;

    Ok(())
}

pub async fn schema_drop(client: &Client) -> Result<(), Error> {
    println!("\nDropping the schema...\n");

    client.batch_execute("DROP TABLE alliances CASCADE").await?;
    client.batch_execute("DROP TABLE guilds CASCADE").await?;
    client.batch_execute("DROP TABLE players").await?;
    client.batch_execute("DROP TABLE cached_events").await?;

    Ok(())
}
