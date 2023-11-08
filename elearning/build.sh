cargo make clean
cargo make build
cargo make dev-deploy
export teacher1=ora-nft.testnet
export USER=kaito021201.testnet
export USER_CT=dev-1698737881732-38966773914805
cargo make call-self init
cargo make call-self update_user_ct_address "{\"user_address\" : \"$USER_CT\"}"
