use diesel::prelude::*;
use std::path::Path;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::cores)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Core {
    pub id: i32,

    /// The name of this core.
    pub name: String,

    /// The slug of this core.
    pub slug: String,

    /// Overwritten name by the user.
    pub version: String,

    /// The path to the core's image.
    pub path: String,

    /// A list of comma-separated authors of the form "Author Name <email@address>".
    pub author: String,

    /// A description of the core.
    pub description: String,

    /// When this core was added to the database.
    pub released_at: chrono::NaiveDateTime,

    /// The last time this core was played.
    pub last_played: Option<chrono::NaiveDateTime>,

    /// Whether this core is a favorite.
    pub favorite: bool,

    /// The last time this core was updated.
    pub downloaded_at: chrono::NaiveDateTime,
}

impl Core {
    pub fn create(
        conn: &mut crate::Connection,
        core: &retronomicon_dto::cores::CoreListItem,
        release: &retronomicon_dto::cores::releases::CoreReleaseRef,
        file_path: impl AsRef<Path>,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::cores;
        use crate::schema::cores::dsl::*;

        diesel::insert_into(cores::table)
            .values((
                name.eq(&core.name),
                slug.eq(&core.slug),
                version.eq(&release.version),
                path.eq(file_path.as_ref().to_str().unwrap()),
                author.eq(&core.owner_team.slug),
                description.eq(&""),
                released_at.eq(
                    chrono::NaiveDateTime::from_timestamp_opt(release.date_released, 0).unwrap(),
                ),
                downloaded_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        cores.order(id.desc()).first(conn)
    }

    pub fn get(conn: &mut crate::Connection, id: i32) -> Result<Self, diesel::result::Error> {
        crate::schema::cores::table.find(id).first(conn)
    }

    pub fn has(
        conn: &mut crate::Connection,
        slug: &str,
        version: &str,
    ) -> Result<bool, diesel::result::Error> {
        crate::schema::cores::table
            .filter(crate::schema::cores::dsl::slug.eq(slug))
            .filter(crate::schema::cores::dsl::version.eq(version))
            .count()
            .get_result::<i64>(conn)
            .map(|c| c > 0)
    }

    pub fn list(conn: &mut crate::Connection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::cores::dsl::*;
        cores.load::<Self>(conn)
    }
}
