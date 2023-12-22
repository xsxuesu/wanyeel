use uuid::Uuid;
use rand::{Rng, thread_rng};

pub fn generate_code()-> String{
    Uuid::new_v4().to_string()
}

pub fn generate_company_code()-> String{
    let company_code_index = "S";
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..=999999);
    let formatted_number = format!("{:06}", random_number);
    return format!("{}{}", company_code_index, formatted_number);
}