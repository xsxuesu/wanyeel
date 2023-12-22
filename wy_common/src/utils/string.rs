use random_number::random;

/**
*struct:IsEmpty
*desc:String 工具类
*author:String
*email:249608904@qq.com
*/
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        return match self {
            Some(s) => s.is_empty(),
            _ => true,
        };
    }
}


pub fn random_code() -> String {
    let mut code = String::new();
    for _ in 0..4{
        let mut rng = random_number::rand::thread_rng();
        let n: u8 = random!(..=9, rng);
        code = format!("{}{}", code,n)
    }
    code
}