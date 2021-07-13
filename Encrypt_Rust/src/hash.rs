fn hash__unit_1(n: u64) -> u64 {
    (n % 20011 + 9109) * (n % 20021 + 9218) / 0x40 % 0x10_000
}


fn hash__unit_2(n: u64) -> u64 {
    (n % 20023 + 9327) * (n % 20029 + 9436) / 0x40 % 0x10_000
}


fn hash__unit_3(n: u64) -> u64 {
    (n % 20047 + 9545) * (n % 20051 + 9654) / 0x40 % 0x8_000
}

fn hash__unit_4(n: u64) -> u64 {
    (n % 20063 + 9763) * (n % 20071 + 9872) / 0x40 % 0x8_000
}


pub fn hash_1(n: u64) -> u64 {
    hash__unit_4(n) * 0x800_000_000_000 + hash__unit_3(n) * 0x100_000_000 + hash__unit_2(n) * 0x10_000 + hash__unit_1(n)
}


fn hash__unit_5(n: u64) -> u64 {
    (n % 30011 + 7107) * (n % 30013 + 7214) / 0x40 % 0x10_000
}


fn hash__unit_6(n: u64) -> u64 {
    (n % 30029 + 7321) * (n % 30047 + 7428) / 0x40 % 0x10_000
}


fn hash__unit_7(n: u64) -> u64 {
    (n % 30059 + 7535) * (n % 30071 + 7642) / 0x40 % 0x8_000
}

fn hash__unit_8(n: u64) -> u64 {
    (n % 30089 + 7749) * (n % 30091 + 7856) / 0x40 % 0x8_000
}


pub fn hash_2(n: u64) -> u64 {
    hash__unit_8(n) * 0x800_000_000_000 + hash__unit_7(n) * 0x100_000_000 + hash__unit_6(n) * 0x10_000 + hash__unit_5(n)
}


pub fn hash_password_1(password: &Vec<u8>) -> u64 {

    let mut result: u64 = 0;

    for (i, v) in password.iter().enumerate() {
        result += hash_1(hash_1(*v as u64) + i as u64);
        result %= 0x4_000_000_000_000_000;
    }

    return result
}


pub fn hash_password_2(password: &Vec<u8>) -> u64 {

    let mut result: u64 = 0;

    for (i, v) in password.iter().enumerate() {
        result += hash_2(hash_2(*v as u64) + i as u64);
        result %= 0x4_000_000_000_000_000;
    }

    return result
}