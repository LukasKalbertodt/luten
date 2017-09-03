//! Declarations for custom SQL types.
//!
//! Right now, `diesel` doesn't  directly support custom SQL types, e.g. `enum`
//! types. In order to use those, we have to write a bit of boilerplate code. I
//! hope that this code can be deleted in the future.

use diesel::pg::Pg;
use diesel::query_source::Queryable;
use diesel::types::*;
use diesel::row::Row;
use std::error::Error;
use std::io::Write;

use user::Role;

// This struct represents the SQL type 'user_role' in PG. Its Rust definition
// is `user::Role`.
#[derive(Debug)]
pub struct UserRole;

impl HasSqlType<UserRole> for Pg {
    fn metadata(lookup: &Self::MetadataLookup) -> Self::TypeMetadata {
        lookup.lookup_type("user_role")
    }
}

impl NotNull for UserRole {}

impl FromSql<UserRole, Pg> for Role {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error + Send + Sync>> {
        match bytes {
            Some(b"admin") => Ok(Role::Admin),
            Some(b"tutor") => Ok(Role::Tutor),
            Some(b"student") => Ok(Role::Student),
            Some(_) => Err("Invalid role variant".into()),
            None => Err("Unexpected null for non-null column".into()),
        }
    }
}

impl ToSql<UserRole, Pg> for Role {
    fn to_sql<W: Write>(
        &self,
        out: &mut ToSqlOutput<W, Pg>,
    ) -> Result<IsNull, Box<Error + Send + Sync>> {
        let bytes: &'static [u8] = match *self {
            Role::Admin => b"admin",
            Role::Tutor => b"tutor",
            Role::Student => b"student",
        };
        out.write_all(bytes)
            .map(|_| IsNull::No)
            .map_err(|e| e.into())
    }
}

impl FromSqlRow<UserRole, Pg> for Role {
    fn build_from_row<R: Row<Pg>>(row: &mut R) -> Result<Self, Box<Error + Send + Sync>> {
        FromSql::<UserRole, Pg>::from_sql(row.take())
    }
}

impl Queryable<UserRole, Pg> for Role {
    type Row = Self;
    fn build(row: Self::Row) -> Self {
        row
    }
}

expression_impls!(UserRole -> Role);
