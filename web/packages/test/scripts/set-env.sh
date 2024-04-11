root_dir="$(realpath ../../..)"
web_dir="$root_dir/web"
lodestar_version="${LODESTAR_VER:-1.12.0}"
geth_version="${GETH_VER:-v1.13.5}"
config_dir="$web_dir/packages/test/config"
export contract_dir="$root_dir/contracts"
test_helpers_dir="$web_dir/packages/test-helpers"
relay_dir="$root_dir/relayer"
relay_bin="$relay_dir/build/snowbridge-relay"
export output_dir="${OUTPUT_DIR:-/tmp/snowbridge}"
export output_bin_dir="$output_dir/bin"
ethereum_data_dir="$output_dir/ethereum"
zombienet_data_dir="$output_dir/zombienet"
export PATH="$output_bin_dir:$PATH"

eth_network="${ETH_NETWORK:-localhost}"
eth_endpoint_http="${ETH_RPC_ENDPOINT:-http://127.0.0.1:8545}/${INFURA_PROJECT_ID:-}"
eth_endpoint_ws="${ETH_WS_ENDPOINT:-ws://127.0.0.1:8546}/${INFURA_PROJECT_ID:-}"
eth_gas_limit="${ETH_GAS_LIMIT:-5000000}"
eth_chain_id="${ETH_NETWORK_ID:-15}"
eth_fast_mode="${ETH_FAST_MODE:-true}"
etherscan_api_key="${ETHERSCAN_API_KEY:-}"
rebuild_lodestar="${REBUILD_LODESTAR:-true}"

parachain_relay_primary_gov_eth_key="${PARACHAIN_RELAY_PRIMARY_GOV_ETH_KEY:-0x8013383de6e5a891e7754ae1ef5a21e7661f1fe67cd47ca8ebf4acd6de66879a}"
parachain_relay_secondary_gov_eth_key="${PARACHAIN_RELAY_SECONDARY_GOV_ETH_KEY:-0xe699de86629f0e795b27e26b33c343876f9282c821a62086b21aef0baa7d7ca7}"
parachain_relay_assethub_eth_key="${PARACHAIN_RELAY_ASSETHUB_ETH_KEY:-0x3646505e08a0f3a61417d91db18f2911b2d17c01563044f3e2f106bf36679a6a}"
parachain_relay_penpal_eth_key="${PARACHAIN_RELAY_PENPAL_ETH_KEY:-0xf9d99a8f29860f6c42714013bae3cf5e833966bf11817f7650a351790e502a45}"
beefy_relay_eth_key="${BEEFY_RELAY_ETH_KEY:-0x935b65c833ced92c43ef9de6bff30703d941bd92a2637cb00cfad389f5862109}"

# Parachain accounts for which the relayer will relay messages over the basic channel.
# These IDs are for the test accounts Alice, Bob, Charlie, Dave, Eve and Ferdie, in order
basic_parachain_account_ids="${BASIC_PARACHAIN_ACCOUNT_IDS:-0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d,0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48,0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22,0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20,0xe659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e,0x1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c}"
# Ethereum addresses for which the relayer will relay messages over the basic channel.
# This address is for the default eth account used in the E2E tests, taken from test/src/ethclient/index.js.
basic_eth_addresses="${BASIC_ETH_ADDRESSES:-0x89b4ab1ef20763630df9743acf155865600daff2}"
beacon_endpoint_http="${BEACON_HTTP_ENDPOINT:-http://127.0.0.1:9596}"

# Local substrate chain endpoints
bridgehub_ws_url="${BRIDGE_HUB_WS_URL:-ws://127.0.0.1:11144}"
bridgehub_seed="${BRIDGE_HUB_SEED:-//Alice}"
bridgehub_pallets_owner="${BRIDGE_HUB_PALLETS_OWNER:-0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d}"
export BRIDGE_HUB_PARAID="${BRIDGE_HUB_PARAID:-1013}"
export BRIDGE_HUB_AGENT_ID="${BRIDGE_HUB_AGENT_ID:-0x03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314}"

assethub_ws_url="${ASSET_HUB_WS_URL:-ws://127.0.0.1:12144}"
assethub_seed="${ASSET_HUB_SEED:-//Alice}"
export ASSET_HUB_PARAID="${ASSET_HUB_PARAID:-1000}"
export ASSET_HUB_AGENT_ID="${ASSET_HUB_AGENT_ID:-0x81c5ab2571199e3188135178f3c2c8e2d268be1313d029b30f534fa579b69b79}"
export ASSET_HUB_CHANNEL_ID="0xc173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539"
export PENPAL_CHANNEL_ID="0xa69fbbae90bb6096d59b1930bbcfc8a3ef23959d226b1861deb7ad8fb06c6fa3"
export PRIMARY_GOVERNANCE_CHANNEL_ID="0x0000000000000000000000000000000000000000000000000000000000000001"
export SECONDARY_GOVERNANCE_CHANNEL_ID="0x0000000000000000000000000000000000000000000000000000000000000002"
export PENPAL_PARAID="${PENPAL_PARAID:-2000}"

# Token decimal of the relaychain(KSM|ROC:12,DOT:10)
export FOREIGN_TOKEN_DECIMALS=12

relaychain_ws_url="${RELAYCHAIN_WS_URL:-ws://127.0.0.1:9944}"
relaychain_sudo_seed="${RELAYCHAIN_SUDO_SEED:-//Alice}"

skip_relayer="${SKIP_RELAYER:-false}"
reset_ethereum="${RESET_ETHEREUM:-true}"

## Important accounts

# Useful tool to get these account values: https://www.shawntabrizi.com/substrate-js-utilities/
# Account for bridgehub (Sibling parachain 1013 in testnet)
bridgehub_sovereign_account="${BRIDGEHUB_SOVEREIGN_ACCOUNT:-0x7369626cf5030000000000000000000000000000000000000000000000000000}"
# Account for assethub (Sibling parachain 1000 5Eg2fntNprdN3FgH4sfEaaZhYtddZQSQUqvYJ1f2mLtinVhV in testnet)
assethub_sovereign_account="${ASSETHUB_SOVEREIGN_ACCOUNT:-0x7369626ce8030000000000000000000000000000000000000000000000000000}"
# Account for penpal (Sibling parachain 2000 5Eg2fntJ27qsari4FGrGhrMqKFDRnkNSR6UshkZYBGXmSuC8 in testnet)
penpal_sovereign_account="${PENPAL_SOVEREIGN_ACCOUNT:-0x7369626cd0070000000000000000000000000000000000000000000000000000}"
# Beacon relay account (//BeaconRelay 5GWFwdZb6JyU46e6ZiLxjGxogAHe8SenX76btfq8vGNAaq8c in testnet)
beacon_relayer_pub_key="${BEACON_RELAYER_PUB_KEY:-0xc46e141b5083721ad5f5056ba1cded69dce4a65f027ed3362357605b1687986a}"
# Execution relay account (//ExecutionRelayAssetHub 5DF6KbMTBPGQN6ScjqXzdB2ngk5wi3wXvubpQVUZezNfM6aV in testnet)
execution_relayer_assethub_pub_key="${EXECUTION_RELAYER_ASSETHUB_PUB_KEY:-0x34284f0c1920694dd2e798e94e378307980d0f52d556009fc451c08bd65a8b4a}"
# Execution relay account (//ExecutionRelayPenpal 5HgmfVcc8xBUcReNJsUaJMhFWGkdYpEw4RiCX4SeZPdKXR6H in testnet)
execution_relayer_penpal_pub_key="${EXECUTION_RELAYER_PENPAL_PUB_KEY:-0xf8aed1861e571ef861cf34fe9587a211465aa380787e0102c2e89dfb0b666d3b}"
# Sovereign account for msg.sender in penpal (Ethereum address:0x90A987B944Cb1dCcE5564e5FDeCD7a54D3de27Fe)
sender_sovereign_account="ee99e7e8ac49f08251154c033f827541f4fb8a5b1fc4d6d9b1ab72c103bd3023"

# Config for deploying contracts
## Deployment key
export PRIVATE_KEY="${DEPLOYER_ETH_KEY:-0x4e9444a6efd6d42725a250b650a781da2737ea308c839eaccb0f7f3dbd2fea77}"
## BeefyClient
# For max safety delay should be MAX_SEED_LOOKAHEAD=4 epochs=4*8*6=192s
# but for rococo-local each session is only 20 slots=120s
# so relax somehow here just for quick test
# for production deployment ETH_RANDAO_DELAY should be configured in a more reasonable sense
export RANDAO_COMMIT_DELAY="${ETH_RANDAO_DELAY:-3}"
export RANDAO_COMMIT_EXP="${ETH_RANDAO_EXP:-3}"
export MINIMUM_REQUIRED_SIGNATURES="${MINIMUM_REQUIRED_SIGNATURES:-16}"

export REJECT_OUTBOUND_MESSAGES="${REJECT_OUTBOUND_MESSAGES:-false}"

## Fee
export REGISTER_TOKEN_FEE="${REGISTER_TOKEN_FEE:-200000000000000000}"
export CREATE_ASSET_FEE="${CREATE_ASSET_FEE:-10000000000}"
export RESERVE_TRANSFER_FEE="${RESERVE_TRANSFER_FEE:-10000000000}"

## Pricing Parameters
export EXCHANGE_RATE="${EXCHANGE_RATE:-2500000000000000}"
export DELIVERY_COST="${DELIVERY_COST:-10000000000}"
export FEE_MULTIPLIER="${FEE_MULTIPLIER:-1000000000000000000}"
export FEE_PER_GAS="${FEE_PER_GAS:-20000000000}"

## Reward
export LOCAL_REWARD="${LOCAL_REWARD:-1000000000000}"
export REMOTE_REWARD="${REMOTE_REWARD:-1000000000000000}"

## Vault
export BRIDGE_HUB_INITIAL_DEPOSIT="${ETH_BRIDGE_HUB_INITIAL_DEPOSIT:-10000000000000000000}"

export GATEWAY_STORAGE_KEY="${GATEWAY_STORAGE_KEY:-0xaed97c7854d601808b98ae43079dafb3}"
export GATEWAY_PROXY_CONTRACT="${GATEWAY_PROXY_CONTRACT:-0xEDa338E4dC46038493b885327842fD3E301CaB39}"

address_for() {
    jq -r ".contracts.${1}.address" "$output_dir/contracts.json"
}

kill_all() {
    trap - SIGTERM
    kill 0
}

cleanup() {
    echo "Cleaning resource"
    rm -rf "$output_dir"
    mkdir "$output_dir"
    mkdir "$output_bin_dir"
    mkdir "$ethereum_data_dir"
}

check_tool() {
    if ! [ -x "$(command -v g++)" ]; then
        echo 'Error: g++ is not installed.'
        exit
    fi
    if ! [ -x "$(command -v protoc)" ]; then
        echo 'Error: protoc is not installed.'
        exit
    fi
    if ! [ -x "$(command -v jq)" ]; then
        echo 'Error: jq is not installed.'
        exit
    fi
    if ! [ -x "$(command -v sponge)" ]; then
        echo 'Error: sponge is not installed.'
        exit
    fi
    if ! [ -x "$(command -v direnv)" ]; then
        echo 'Error: direnv is not installed.'
        exit
    fi
    if ! [ -x "$(command -v mage)" ]; then
        echo 'Error: mage is not installed.'
        exit
    fi
    if ! [ -x "$(command -v pnpm)" ]; then
        echo 'Error: pnpm is not installed.'
        exit
    fi
}

wait_contract_deployed() {
    local ready=""
    while [ -z "$ready" ]; do
        if [ -f "$output_dir/contracts.json" ]; then
            ready="true"
        fi
        sleep 2
    done
}
