use alloy_multicall::Multicall;
use alloy_sol_types::{sol, JsonAbiExt};
use alloy_primitives::{address, U256};
use alloy_dyn_abi::DynSolValue;




sol! {
    #[derive(Debug)]
    #[sol(abi)]
    function getAmountsOut(uint amountIn, address[] memory path)
        public
        view
        virtual
        override
        returns (uint[] memory amounts);
}




#[tokio::main]
async fn main() {
    let rpc_url = "https://rpc.ankr.com/eth".parse().unwrap();
    let provider = alloy_provider::ProviderBuilder::new().on_http(rpc_url);  
    let uniswap_v2 = address!("7a250d5630b4cf539739df2c5dacb4c659f2488d");


    let mut multicall = Multicall::new(provider.clone(), None).await.unwrap();

    
    let amounts_out = getAmountsOutCall::abi();

    multicall.add_call(uniswap_v2, &amounts_out, &[
        DynSolValue::from(U256::from(1000000000000000000_u128)),
        DynSolValue::Array(vec![
            address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").into(), 
            address!("6982508145454Ce325dDbE47a25d4ec3d2311933").into()
        ]),
    ], false);


    let results = multicall.call().await.unwrap();
    println!("{:?}", results);
}
