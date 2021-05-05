use std::fs::read;


#[derive(Copy, Clone, Debug)]
enum operation_type {
    FLIP,
    NOT,
    LSHIFT,
    RSHIFT
}


#[derive(Copy, Clone, Debug)]
struct operation {
    n: usize,
    op: operation_type
}


/*
bit 수준에서 섞으니까 임시로 Vec<bool>을 갖고 있다가 8개씩 끊어서 Vec<u8>로 바꿔주는 거임
*/
fn do_ops(target: &Vec<u8>, ops: &Vec<operation>) -> Vec<u8> {

    let mut result_cache: Vec<bool> = vec![];
    let mut result: Vec<u8> = Vec::with_capacity(target.len());

    // not encrypted yet
    let mut target_cache: Vec<bool> = vec![];

    // target[curr_index]부터 읽으면 됨
    let mut curr_index: usize = 0;
    let target_length = target.len();

    let mut ops_index: usize = 0;
    let ops_length = ops.len();

    loop {

        while target_cache.len() < 32 && curr_index < target_length {
            target_cache.extend(u8_to_bools(target[curr_index]));
            curr_index += 1;
        }

        while result_cache.len() > 8 {
            result.push(bools_to_u8(&result_cache));
            result_cache = result_cache[8..].to_vec();
        }

        let n = ops[ops_index].n;

        if curr_index == target_length && target_cache.len() < n {
            result_cache.extend(target_cache);
            break;
        }

        match ops[ops_index].op {

            operation_type::FLIP => {
                result_cache.extend(op_flip(&target_cache[..n].to_vec()));
                target_cache = target_cache[n..].to_vec();
            },

            operation_type::NOT => {
                result_cache.extend(op_not(&target_cache[..n].to_vec()));
                target_cache = target_cache[n..].to_vec();
            },

            operation_type::LSHIFT => {
                result_cache.extend(op_lshift(&target_cache[..n].to_vec()));
                target_cache = target_cache[n..].to_vec();
            },

            operation_type::RSHIFT => {
                result_cache.extend(op_rshift(&target_cache[..n].to_vec()));
                target_cache = target_cache[n..].to_vec();
            }

        }

        ops_index = (ops_index + 1) % ops_length;
    }

    while result_cache.len() >= 8 {
        result.push(bools_to_u8(&result_cache));
        result_cache = result_cache[8..].to_vec();
    }

    result
}


fn gen_ops(password: &Vec<u8>) -> Vec<operation> {

    let mut result: Vec<operation> = vec![];

    let operation_types = vec![operation_type::FLIP, operation_type::NOT, operation_type::LSHIFT, operation_type::RSHIFT];

    // length를 최대한 8의 배수랑 안 친하게 써야 됨
    // 아무렇게나 2||3개 더해도 8의 배수가 안 됨!
    let ns: Vec<usize> = vec![7, 11, 15];

    let mut hashed = hash_password(password);
    let mut curr_val = hashed;

    let mut n_sum: usize = 0;

    for _ in 0..5 {

        while curr_val > 12 {
            let n = ns[(curr_val % 3) as usize];
            let op_type = operation_types[(curr_val % 4) as usize];

            result.push(
                operation {
                    n: n,
                    op: op_type
                }
            );

            n_sum += n;
            curr_val /= 12;
        }

        hashed = hash(hashed);
        curr_val = hashed;
    }

    // byte 단위로 딱 맞게 잘리면 좀 이상하지??
    if n_sum % 8 == 0 {
        result.push(
            operation {
                n: 7,
                op: operation_type::FLIP
            }
        );
    }

    result
}


/*
암호화나 복호화나 다 똑같은데
RSHIFT랑 LSHIFT만 반대로 해주면 됨
*/
fn reverse_ops(ops: &Vec<operation>) -> Vec<operation> {
    ops.iter().map(
        |o|
        match o.op {
            operation_type::RSHIFT => operation {
                n: o.n,
                op: operation_type::LSHIFT
            },
            operation_type::LSHIFT => operation {
                n: o.n,
                op: operation_type::RSHIFT
            },
            _ => o.clone()
        }
    ).collect()
}


// 이러면 most significant bit가 제일 오른쪽으로 가지 않음??
// 걍 쓸까?? 암호화/복호화에는 별 문제 없을 거 같은디...
fn u8_to_bools(mut from: u8) -> Vec<bool> {

    let mut result: Vec<bool> = Vec::with_capacity(8);

    for _ in 0..8 {
        result.push(from % 2 != 0);
        from /= 2;
    }

    result
}


// Debug function
fn string_to_bytestring(from: &Vec<u8>) -> String {

    from.iter().map(|n| bools_to_bytestring(&u8_to_bools(*n))).collect::<Vec<String>>().join("")
}


// Debug function
fn bools_to_bytestring(from: &Vec<bool>) -> String {

    from.iter().map(|b| if *b {String::from("1")} else {String::from("0")} ).collect::<Vec<String>>().join("")
}


// it only reads the first 8 booleans
// the most significant bit is at the right side
fn bools_to_u8(from: &Vec<bool>) -> u8 {

    (
        from[0] as u8
    ) * 1 + (
        from[1] as u8
    ) * 2 + (
        from[2] as u8
    ) * 4 + (
        from[3] as u8
    ) * 8 + (
        from[4] as u8
    ) * 16 + (
        from[5] as u8
    ) * 32 + (
        from[6] as u8
    ) * 64 + (
        from[7] as u8
    ) * 128
}


fn hash1(n: u64) -> u64 {
    (n % 20011 + 9109) * (n % 20021 + 9218) / 0x40 % 0x10_000
}


fn hash2(n: u64) -> u64 {
    (n % 20023 + 9327) * (n % 20029 + 9436) / 0x40 % 0x10_000
}


fn hash3(n: u64) -> u64 {
    (n % 20047 + 9545) * (n % 20051 + 9654) / 0x40 % 0x8_000
}

fn hash4(n: u64) -> u64 {
    (n % 20063 + 9763) * (n % 20071 + 9872) / 0x40 % 0x8_000
}


fn hash(n: u64) -> u64 {
    hash4(n) * 0x800_000_000_000 + hash3(n) * 0x100_000_000 + hash2(n) * 0x10_000 + hash1(n)
}


fn hash_password(password: &Vec<u8>) -> u64 {

    let mut result: u64 = 0;

    for (i, v) in password.iter().enumerate() {
        result += hash(hash(*v as u64) + i as u64);
        result %= 0x4_000_000_000_000_000;
    }

    return result
}


fn op_not(src: &Vec<bool>) -> Vec<bool> {
    src.iter().map(|b| !b).collect()
}


fn op_flip(src: &Vec<bool>) -> Vec<bool> {
    (0..src.len()).map(|i| src[src.len() - i - 1]).collect()
}


fn op_lshift(src: &Vec<bool>) -> Vec<bool> {

    let mut result: Vec<bool> = vec![true;src.len()];

    for i in 1..src.len() {
        result[i] = src[i - 1];
    }

    result[0] = src[src.len() - 1];

    result
}


fn op_rshift(src: &Vec<bool>) -> Vec<bool> {

    let mut result: Vec<bool> = vec![true;src.len()];

    for i in 0..src.len() - 1 {
        result[i] = src[i + 1];
    }

    result[src.len() - 1] = src[0];

    result
}


fn _encrypt(src: &Vec<u8>, password: &Vec<u8>) -> Vec<u8> {
    do_ops(src, &gen_ops(password))
}


fn _decrypt(src: &Vec<u8>, password: &Vec<u8>) -> Vec<u8> {
    do_ops(src, &reverse_ops(&gen_ops(password)))
}


fn encrypt_file(src: String, password: String) -> Result<Vec<u8>, std::io::Error> {
    let data = read(src)?;

    return Ok(_encrypt(&data, &password.into_bytes()));
}


fn decrypt_file(src: String, password: String) -> Result<Vec<u8>, std::io::Error> {
    let data = read(src)?;

    return Ok(_decrypt(&data, &password.into_bytes()));
}


fn main() {
    println!("gogo");
    std::fs::write("result.t", encrypt_file(String::from("main.rs"), String::from("1q2w3e4r")).unwrap());
    println!("done");
}
