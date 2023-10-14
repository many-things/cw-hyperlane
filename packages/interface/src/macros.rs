#[macro_export]
macro_rules! build_test_querier {
    ($handler:expr) => {
        fn test_querier<
            S: cosmwasm_schema::serde::Serialize,
            T: cosmwasm_schema::serde::de::DeserializeOwned,
        >(
            deps: cosmwasm_std::Deps,
            msg: S,
        ) -> T {
            let res = $handler(
                deps,
                cosmwasm_std::testing::mock_env(),
                cosmwasm_std::from_binary(&cosmwasm_std::to_binary(&msg).unwrap()).unwrap(),
            )
            .map_err(|e| e.to_string())
            .unwrap();
            cosmwasm_std::from_binary(&res).unwrap()
        }
    };
}
