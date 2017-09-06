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
use state::AppState as RealAppState;
use timeslot::DayOfWeek as RealDayOfWeek;

/// Macro to generate boilerplate code needed to use a postgres enum types.
///
/// Parameters:
/// 1. `pg_type`: a string literal containing the type name in postgres, e.g.
///               "user_role".
/// 2. `diesel_ty`: a name for a dummy type representing the postgres type.
///                 This is the type used in the `table!` macro. You can choose
///                 it however you like, but it should be the camel-cased form
///                 of `pg_type`.
/// 3. `real_ty`: the name of your real enum type.
///
/// After those three parameters, a list of variant-"postgres value" pairs is
/// given.
macro_rules! enum_pg_type {
    (
        $pg_type:expr,
        $diesel_ty:ident,
        $real_ty:ident;
        {
            $($variant:ident => $pg_val:expr , )+
        }
    ) => {
        // This struct represents the SQL type 'user_role' in PG. Its Rust definition
        // is `user::Role`.
        #[derive(Debug)]
        pub struct $diesel_ty;

        impl HasSqlType<$diesel_ty> for Pg {
            fn metadata(lookup: &Self::MetadataLookup) -> Self::TypeMetadata {
                lookup.lookup_type($pg_type)
            }
        }

        impl NotNull for $diesel_ty {}

        impl FromSql<$diesel_ty, Pg> for $real_ty {
            fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error + Send + Sync>> {
                match bytes {
                    $(
                        Some($pg_val) => Ok($real_ty::$variant),
                    )+
                    Some(_) => Err("Invalid variant".into()),
                    None => Err("Unexpected null for non-null column".into()),
                }
            }
        }

        impl ToSql<$diesel_ty, Pg> for $real_ty {
            fn to_sql<W: Write>(
                &self,
                out: &mut ToSqlOutput<W, Pg>,
            ) -> Result<IsNull, Box<Error + Send + Sync>> {
                let bytes: &'static [u8] = match *self {
                    $(
                        $real_ty::$variant => $pg_val,
                    )+
                };
                out.write_all(bytes)
                    .map(|_| IsNull::No)
                    .map_err(|e| e.into())
            }
        }

        impl FromSqlRow<$diesel_ty, Pg> for $real_ty {
            fn build_from_row<R: Row<Pg>>(row: &mut R) -> Result<Self, Box<Error + Send + Sync>> {
                FromSql::<$diesel_ty, Pg>::from_sql(row.take())
            }
        }

        impl Queryable<$diesel_ty, Pg> for $real_ty {
            type Row = Self;
            fn build(row: Self::Row) -> Self {
                row
            }
        }

        expression_impls!($diesel_ty -> $real_ty);
    }
}

enum_pg_type!("user_role", UserRole, Role; {
    Admin => b"admin",
    Tutor => b"tutor",
    Student => b"student",
});

enum_pg_type! ("app_state", AppState, RealAppState; {
    Preparation => b"preparation",
    Running => b"running",
    Frozen => b"frozen",
});

enum_pg_type! ("day_of_week", DayOfWeek, RealDayOfWeek; {
    Monday => b"monday",
    Tuesday => b"tuesday",
    Wednesday => b"wednesday",
    Thursday => b"thursday",
    Friday => b"friday",
    Saturday => b"saturday",
    Sunday => b"sunday",
});
