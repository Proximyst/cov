//! This module contains the models for the database schema.
//! The represented models here are the latest version of the schema as defined in the migrations.
//!
//! To understand them, you might be interested in these resources:
//!   - Run `just test-cov` and look through target/llvm-cov/.
//!   - https://github.com/jacoco/jacoco/blob/0bb8bd525f22c6d2427c3b5e23b0292ec64201b0/org.jacoco.report/src/org/jacoco/report/xml/report.dtd
//!   - https://github.com/golang/go/blob/0104a31b8fbcbe52728a08867b26415d282c35d2/src/cmd/cover/profile.go#L53-L56

use chrono::{DateTime, Utc};
use std::marker::PhantomData;
use uuid::Uuid;

/// Id is a type-safe wrapper around an ID of a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id<I, M> {
    pub id: I,
    model: PhantomData<M>,
}

pub type BigIntId<M> = Id<i64, M>;
pub type UuidId<M> = Id<Uuid, M>;

macro_rules! services {
    ($enum:ident => $($(#[$meta:meta])* $name:ident = $str:literal),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $enum {
            $($(#[$meta])* $name,)*
        }

        impl $enum {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$name => $str,)*
                }
            }
        }

        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::str::FromStr for $enum {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($str => Ok(Self::$name),)*
                    _ => Err(()),
                }
            }
        }

        impl tokio_postgres::types::ToSql for $enum {
            fn to_sql(
                &self,
                ty: &tokio_postgres::types::Type,
                out: &mut bytes::BytesMut,
            ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                self.as_str().to_sql(ty, out)
            }

            fn accepts(ty: &tokio_postgres::types::Type) -> bool {
                <&str as tokio_postgres::types::ToSql>::accepts(ty)
            }

            tokio_postgres::types::to_sql_checked!();
        }
    };
}
services! [ Service =>
    /// GitHub.com specifically.
    GitHub = "ghcom",
    /// Self-hosted GitHub Enterprise Server instance.
    GitHubEnterprise = "ghes",
];

#[derive(Debug, Clone)]
pub struct Organisation {
    pub id: BigIntId<Organisation>,
    pub service: Service,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub id: BigIntId<Repository>,
    pub organisation_id: BigIntId<Organisation>,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub id: UuidId<Report>,
    pub repository_id: BigIntId<Repository>,
    pub commit: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ReportFlag {
    pub report_id: UuidId<Report>,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ReportFile {
    pub id: BigIntId<ReportFile>,
    pub repository_id: BigIntId<Repository>,
    pub report_id: UuidId<Report>,
    pub file_path: String,
    pub coverage: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ReportFileLineRegion {
    pub id: BigIntId<ReportFileLineRegion>,
    pub repository_id: BigIntId<Repository>,
    pub report_file_id: BigIntId<ReportFile>,

    pub start_line: i32,   // start_line >= 1
    pub end_line: i32,     // end_line >= start_line
    pub start_column: i32, // start_column >= 0
    pub end_column: i32,   // end_column >= 0

    pub statements: i32, // statements >= 0
    pub executed: i32,   // executed >= 0

    pub created_at: DateTime<Utc>,
}
