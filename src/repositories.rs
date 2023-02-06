use sqlx::mysql::MySqlPool;
use async_trait::async_trait;
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Employee {
    pub id: i8,
    pub name: String,
}

impl Employee {
    pub fn new(id: i8, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Error)]
enum RepositoryErr {
    #[error("Not Found, id is {0}")]
    NotFound(i32),
}

#[async_trait]
pub trait EmployeeRepository: Clone + std::marker::Send
    + std::marker::Sync + 'static {
    async fn create(&self, payload: Employee) -> anyhow::Result<Employee>;
    async fn list(&self) -> anyhow::Result<Vec<Employee>>;
}

#[derive(Debug, Clone)]
pub struct EmployeeRepositoryForDB {
    pub pool: MySqlPool,
}

impl EmployeeRepositoryForDB {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmployeeRepository for EmployeeRepositoryForDB {
    async fn create(&self, payload: Employee) -> anyhow::Result<Employee> {
        todo!()
    }
    async fn list(&self) -> anyhow::Result<Vec<Employee>> {
        let res = sqlx::query_as::<_, Employee>(r#"
            SELECT * FROM employee;
        "#,)
        .fetch_all(&self.pool)
        .await?;
        Ok(res)
    }
}