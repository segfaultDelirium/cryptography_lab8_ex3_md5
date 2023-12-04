use crypto::digest::Digest;

fn hash_vector_to_string(hash_vector: Vec<u8>) -> String{
    hash_vector.into_iter()
        .map(|value| format!("{:x}", value))
        .reduce(|acc, x| {
            format!("{acc}{x}")
        }).unwrap()
}


fn string_to_hash_string_md5(s: &str) -> String {
    let mut hasher = crypto::md5::Md5::new();
    hasher.input_str(s);
    let result = hasher.result_str();
    result
}

fn main() {
    let phrase0 = "The quick brown fox jumps over the lazy dog";
    let phrase1 = "The quick brown fox jumps over the lazy cog";
    let phrase2 = "";

    // println!("MD4");
    // let digest0 = string_to_hash_string_md4(phrase0);
    // println!("digest is: {digest0}");
    //
    // let digest1 = string_to_hash_string_md4(phrase1);
    // println!("digest is: {digest1}");
    //
    // let digest2 = string_to_hash_string_md4(phrase2);
    // println!("digest is: {digest2}");


    println!("MD5");
    let digest0 = string_to_hash_string_md5(phrase0);
    println!("digest is: {digest0}");

    let digest1 = string_to_hash_string_md5(phrase1);
    println!("digest is: {digest1}");

    let digest2 = string_to_hash_string_md5(phrase2);
    println!("digest is: {digest2}");


    println!("bye");
}
