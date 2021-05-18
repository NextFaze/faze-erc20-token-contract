use super::*;

mod lib_tests {
    use erc20::*;
    use ink_lang as ink;

    #[ink::test]
    fn correctly_instantiates() {
        let erc20 = Erc20::new(1000);
        assert_eq!(erc20.get(), 1);
    }
}
