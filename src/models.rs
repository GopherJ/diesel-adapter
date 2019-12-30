use super::schema::casbin_rules;

#[derive(Queryable, Identifiable)]
pub(crate) struct CasbinRule {
    pub id: i32,
    pub ptype: Option<String>,
    pub v0: Option<String>,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}

#[derive(Insertable)]
#[table_name = "casbin_rules"]
pub(crate) struct NewCasbinRule<'a> {
    pub ptype: Option<&'a str>,
    pub v0: Option<&'a str>,
    pub v1: Option<&'a str>,
    pub v2: Option<&'a str>,
    pub v3: Option<&'a str>,
    pub v4: Option<&'a str>,
    pub v5: Option<&'a str>,
}

#[derive(Clone, Debug)]
pub struct ConnOptions<'a> {
    hostname: &'a str,
    port: u16,
    username: Option<&'a str>,
    password: Option<&'a str>,
    database: &'a str,
    table: &'a str,
    pool_size: u8,
}

impl<'a> Default for ConnOptions<'a> {
    fn default() -> Self {
        cfg_if! {
            if #[cfg(feature = "postgres")] {
                ConnOptions {
                    hostname: "127.0.0.1",
                    port: 5432,
                    username: None,
                    password: None,
                    database: "casbin",
                    table: "casbin_rules",
                    pool_size: 8,
                }
            } else if #[cfg(feature = "mysql")] {
                ConnOptions {
                    hostname: "127.0.0.1",
                    port: 3306,
                    username: None,
                    password: None,
                    database: "casbin",
                    table: "casbin_rules",
                    pool_size: 8,
                }
            }
        }
    }
}

impl<'a> ConnOptions<'a> {
    pub fn set_hostname(&mut self, hostname: &'a str) -> &mut Self {
        self.hostname = hostname;
        self
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    fn get_host(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    pub fn set_auth(&mut self, username: &'a str, password: &'a str) -> &mut Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    fn get_auth(&self) -> Option<String> {
        if let (Some(user), Some(pass)) = (self.username, self.password) {
            Some(format!("{}:{}", user, pass))
        } else {
            None
        }
    }

    cfg_if! {
        if #[cfg(feature = "postgres")] {
            pub fn get_url(&self) -> String {
                if let Some(auth) = self.get_auth() {
                    format!("postgres://{}@{}/{}", auth, self.get_host(), self.database)
                } else {
                    format!("postgres://{}/{}", self.get_host(), self.database)
                }
            }
        } else if #[cfg(feature = "mysql")] {
            pub fn get_url(&self) -> String {
                if let Some(auth) = self.get_auth() {
                    format!("mysql://{}@{}/{}", auth, self.get_host(), self.database)
                } else {
                    format!("mysql://{}/{}", self.get_host(), self.database)
                }
            }
        }
    }

    pub fn get_table(&self) -> String {
        self.table.to_owned()
    }

    pub fn set_table(&mut self, table: &'a str) -> &mut Self {
        self.table = table;
        self
    }

    pub fn set_pool(&mut self, pool_size: u8) -> &mut Self {
        self.pool_size = pool_size;
        self
    }

    pub fn get_db(&self) -> String {
        self.database.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "postgres")]
    #[test]
    fn test_url() {
        use super::*;
        let mut conn_options = ConnOptions::default();
        conn_options.set_auth("test", "test");

        assert_eq!(
            "postgres://test:test@127.0.0.1:5432/casbin",
            conn_options.get_url()
        )
    }

    #[cfg(feature = "mysql")]
    #[test]
    fn test_url() {
        use super::*;
        let conn_options = ConnOptions::default();
        assert_eq!("mysql://127.0.0.1:3306/casbin", conn_options.get_url())
    }
}
