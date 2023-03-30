// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.19;

import "canonical-weth/WETH9.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/BeefyClient.sol";
import "../src/ParachainClient.sol";
import "../src/InboundQueue.sol";
import "../src/OutboundQueue.sol";
import "../src/NativeTokens.sol";
import "../src/Vault.sol";
import "../src/SovereignTreasury.sol";
import "../src/IVault.sol";

contract DeployScript is Script {
    function setUp() public {}

    function run() public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(privateKey);
        vm.startBroadcast(deployer);

        // SovereignTreasury
        Vault vault = new Vault();
        SovereignTreasury treasury = new SovereignTreasury(vault);

        // BeefyClient
        uint256 randaoCommitDelay = vm.envUint("RANDAO_COMMIT_DELAY");
        uint256 randaoCommitExpiration = vm.envUint("RANDAO_COMMIT_EXP");
        BeefyClient beefyClient = new BeefyClient(randaoCommitDelay, randaoCommitExpiration);

        // ParachainClient
        uint32 paraId = uint32(vm.envUint("PARAID"));
        ParachainClient parachainClient = new ParachainClient(beefyClient, paraId);

        // InboundQueue
        uint256 relayerReward = vm.envUint("RELAYER_REWARD");
        InboundQueue inboundQueue = new InboundQueue(parachainClient, vault, relayerReward);

        // OutboundQueue
        uint256 relayerFee = vm.envUint("RELAYER_FEE");
        OutboundQueue outboundQueue = new OutboundQueue(vault, relayerFee);

        // NativeTokens
        TokenVault tokenVault = new TokenVault();
        NativeTokens nativeTokens = new NativeTokens(
            tokenVault,
            outboundQueue,
            vm.envBytes("STATEMINT_LOCATION"),
            vm.envUint("CREATE_TOKEN_FEE")
        );

        // Deploy WETH for testing
        new WETH9();

        // Allow inbound channel to send messages to NativeTokens and SovereignTreasury
        nativeTokens.grantRole(nativeTokens.SENDER_ROLE(), address(inboundQueue));
        treasury.grantRole(treasury.SENDER_ROLE(), address(inboundQueue));

        // Allow InboundQueue and SovereignTreasury to withdraw from vault
        vault.grantRole(vault.WITHDRAW_ROLE(), address(inboundQueue));
        vault.grantRole(vault.WITHDRAW_ROLE(), address(treasury));

        // Allow NativeTokens to withdraw from TokenVault
        tokenVault.grantRole(tokenVault.WITHDRAW_ROLE(), address(nativeTokens));
        tokenVault.grantRole(tokenVault.DEPOSIT_ROLE(), address(nativeTokens));


        vm.stopBroadcast();
    }
}