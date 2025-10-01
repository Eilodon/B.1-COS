use pandora_tools::skills::arithmetic_skill::ArithmeticSkill;
use pandora_core::interfaces::skills::SkillModule;
use proptest::prelude::*;
use tokio::runtime::Runtime;
use serde_json::json;

proptest! {
    #[test]
    fn parse_numbers_always_succeeds(n in prop::num::f64::ANY.prop_filter("finite", |x| x.is_finite())) {
        let skill = ArithmeticSkill;
        let input = json!({"expression": n.to_string()});
        let rt = Runtime::new().expect("tokio runtime");
        let result = rt.block_on(skill.execute(input));
        prop_assert!(result.is_ok());
        let output = result.unwrap();
        let parsed = output["result"].as_f64().unwrap();
        prop_assert!((parsed - n).abs() < 1e-6);
    }

    #[test]
    fn addition_commutative(a in -1000..1000, b in -1000..1000) {
        let skill = ArithmeticSkill;
        let expr1 = format!("{} + {}", a, b);
        let expr2 = format!("{} + {}", b, a);
        let rt = Runtime::new().expect("tokio runtime");
        let result1 = rt.block_on(skill.execute(json!({"expression": expr1}))).unwrap()["result"].as_f64().unwrap();
        let result2 = rt.block_on(skill.execute(json!({"expression": expr2}))).unwrap()["result"].as_f64().unwrap();
        prop_assert_eq!(result1, result2);
    }

    #[test]
    fn multiplication_commutative(a in -100..100, b in -100..100) {
        let skill = ArithmeticSkill;
        let expr1 = format!("{} * {}", a, b);
        let expr2 = format!("{} * {}", b, a);
        let rt = Runtime::new().expect("tokio runtime");
        let result1 = rt.block_on(skill.execute(json!({"expression": expr1}))).unwrap()["result"].as_f64().unwrap();
        let result2 = rt.block_on(skill.execute(json!({"expression": expr2}))).unwrap()["result"].as_f64().unwrap();
        prop_assert_eq!(result1, result2);
    }

    #[test]
    fn operator_precedence(a in -100..100, b in -100..100, c in 1..100) {
        let skill = ArithmeticSkill;
        let expr1 = format!("{} + {} * {}", a, b, c);
        let expr2 = format!("{} + ({} * {})", a, b, c);
        let rt = Runtime::new().expect("tokio runtime");
        let result1 = rt.block_on(skill.execute(json!({"expression": expr1}))).unwrap()["result"].as_f64().unwrap();
        let result2 = rt.block_on(skill.execute(json!({"expression": expr2}))).unwrap()["result"].as_f64().unwrap();
        prop_assert_eq!(result1, result2);
    }

    #[test]
    fn division_by_zero_errors(n in -1000..1000) {
        let skill = ArithmeticSkill;
        let expr = format!("{} / 0", n);
        let input = json!({"expression": expr});
        let rt = Runtime::new().expect("tokio runtime");
        let result = rt.block_on(skill.execute(input));
        prop_assert!(result.is_err());
    }

    #[test]
    fn parentheses_change_order(a in 1..10, b in 1..10, c in 1..10) {
        let skill = ArithmeticSkill;
        let without_parens = format!("{} * {} + {}", a, b, c);
        let with_parens = format!("{} * ({} + {})", a, b, c);
        let rt = Runtime::new().expect("tokio runtime");
        let result1 = rt.block_on(skill.execute(json!({"expression": without_parens}))).unwrap()["result"].as_f64().unwrap();
        let result2 = rt.block_on(skill.execute(json!({"expression": with_parens}))).unwrap()["result"].as_f64().unwrap();
        if a > 1 && b > 0 && c > 0 { prop_assert_ne!(result1, result2); }
    }
}


