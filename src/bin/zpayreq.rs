fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let shielded = args.iter().any(|s| s == "--shielded");
    args.retain(|s| s != "--shielded");
    if args.len() != 2 {
        eprintln!("{}", serde_json::json!({"error":"ERR_USAGE"}));
        std::process::exit(2);
    }
    match zpayreq::payment_request(&args[0], &args[1], shielded) {
        Ok(uri) => println!("{uri}"),
        Err(code) => {
            eprintln!("{}", serde_json::json!({"error":code}));
            std::process::exit(1);
        }
    }
}
