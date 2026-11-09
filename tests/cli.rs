use std::process::Command;
const T: &str = "t1Hsc1LR8yKnbbe3twRp88p6vFfC5t7DLbs";
const S: &str = "zs1z7rejlpsa98s2rrrfkwmaxu53e4ue0ulcrw0h4x5g8jl04tak0d3mm47vdtahatqrlkngh9slya";
const U: &str = "u1qpatys4zruk99pg59gcscrt7y6akvl9vrhcfyhm9yxvxz7h87q6n8cgrzzpe9zru68uq39uhmlpp5uefxu0su5uqyqfe5zp3tycn0ecl";
#[test]
fn payment_cli_contract() {
    for address in [S, U] {
        let result = Command::new(env!("CARGO_BIN_EXE_zpayreq"))
            .args([address, "0.00000001", "--shielded"])
            .output()
            .unwrap();
        assert!(result.status.success());
        assert_eq!(
            String::from_utf8(result.stdout).unwrap().trim(),
            format!("zcash:{address}?amount=0.00000001")
        );
    }
    for (args, error) in [
        (vec!["bad", "1"], "ERR_INVALID_ADDRESS"),
        (vec![T, "1", "--shielded"], "ERR_SHIELDED_REQUIRED"),
        (vec![T, "NaN"], "ERR_AMOUNT_FORMAT"),
        (vec![T, "-1"], "ERR_AMOUNT_NON_POSITIVE"),
        (vec![T, "0"], "ERR_AMOUNT_NON_POSITIVE"),
        (vec![T, "0.000000001"], "ERR_AMOUNT_PRECISION"),
        (vec![T, "21000001"], "ERR_AMOUNT_RANGE"),
        (vec![], "ERR_USAGE"),
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_zpayreq"))
            .args(args)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&result.stderr).unwrap()["error"],
            error
        );
    }
}
