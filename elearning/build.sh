export USER=ora-nft.testnet
export USER_CT=dev-1698119068743-99739638773932
cargo make clean
cargo make build
cargo make dev-deploy
cargo make call-self init
cargo make call-self update_user_ct_address "{\"user_address\" : \"$USER_CT\"}"
