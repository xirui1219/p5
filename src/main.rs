use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use sqlite::Error as SqErr;
#[derive(Debug)]
pub enum UBaseErr {
    DbErr(SqErr),
    HashError(BcryptError),
}

fn main() {
    println!("Hello, world!");
}
impl From<SqErr> for UBaseErr {
    fn from(s: SqErr) -> Self {
        UBaseErr::DbErr(s)
    }
}
impl From<BcryptError> for UBaseErr {
    fn from(b: BcryptError) -> Self {
        UBaseErr::HashError(b)
    }
}
impl UserBase {
    pub fn add_user(&self, u_name: &str, p_word: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let hpass = bcrypt::hash(p_word, DEFAULT_COST)?;
        let mut st = conn.prepare("insert into users(u_name, p_word) values (?,?);")?;
        st.bind(1, u_name)?;
        st.bind(2, &hpass as &str)?;
        st.next()?;
        Ok(())
    }
    pub fn pay(&self, u_from: &str, u_to: &str, amount: i64) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare(
            "insert into transactions (u_from, u_to, t_date,
        t_amount) values(?,?,datetime(\"now\"),?);",
        )?;
        st.bind(1, u_from)?;
        st.bind(2, u_to)?;
        st.bind(3, amount)?;
        st.next()?;
        Ok(())
    }
}



pub struct UserBase {
    fname: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_user_test() {
        let db_path = "data/users.db";
        let user_base = UserBase { fname: db_path.to_string() };

        let res = user_base.add_user("new_user", "new_pass");
        assert!(res.is_ok());

    }

    #[test]
    fn pay_test(){
        let db_path = "data/users.db";
        let user_base = UserBase { fname: db_path.to_string() };
        let res = user_base.pay("A", "B", 100);
        assert!(res.is_ok());

    }
}