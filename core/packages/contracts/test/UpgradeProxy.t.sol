// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.19;

import {AccessControl} from "openzeppelin/access/AccessControl.sol";

import {Test} from "forge-std/Test.sol";

import {UpgradeProxy} from "../src/UpgradeProxy.sol";
import {IUpgradeTask} from "../src/IUpgradeTask.sol";

import {IVault, Vault} from "../src/Vault.sol";
import {OutboundQueue} from "../src/OutboundQueue.sol";

import {ParaID} from "../src/Types.sol";

contract UpgradeTask is IUpgradeTask, AccessControl {
    OutboundQueue public immutable outboundQueue;

    constructor(OutboundQueue _outboundQueue) {
        outboundQueue = _outboundQueue;
    }

    // In this simple upgrade we just update a fee parameter
    function run() external {
        outboundQueue.updateFee(2 ether);
    }
}

contract FailingUpgradeTask is IUpgradeTask, AccessControl {
    function run() pure external {
        revert("failed");
    }
}

contract UpgradeProxyTest is Test {
    UpgradeProxy public upgradeProxy;
    IUpgradeTask public upgrader;
    IUpgradeTask public failingUpgrader;

    OutboundQueue public outboundQueue;

    ParaID origin = ParaID.wrap(1001);

    function setUp() public {
        upgradeProxy = new UpgradeProxy(origin);
        outboundQueue = new OutboundQueue(new Vault(), 1 ether);

        outboundQueue.grantRole(outboundQueue.ADMIN_ROLE(), address(upgradeProxy));
        outboundQueue.revokeRole(outboundQueue.ADMIN_ROLE(), address(this));

        upgradeProxy.grantRole(upgradeProxy.SENDER_ROLE(), address(this));

        // create upgrader instances
        upgrader = new UpgradeTask(outboundQueue);
        failingUpgrader = new FailingUpgradeTask();
    }

    function testUpgrade() public {
        // execute upgrader
        bytes memory message = abi.encode(
            UpgradeProxy.Message(
                UpgradeProxy.Action.Upgrade,
                abi.encode(UpgradeProxy.UpgradePayload(address(upgrader)))));
        upgradeProxy.handle(origin, message);

        assertEq(outboundQueue.fee(), 2 ether);
    }

    function testUpgradeFailBadOrigin() public {
        vm.expectRevert(UpgradeProxy.Unauthorized.selector);
        upgradeProxy.handle(ParaID.wrap(3), hex"deadbeef");
    }

    function testUpgradeUpgraderFail() public {
        bytes memory message = abi.encode(
            UpgradeProxy.Message(
                UpgradeProxy.Action.Upgrade,
                abi.encode(UpgradeProxy.UpgradePayload(address(failingUpgrader)))));
        vm.expectRevert(UpgradeProxy.UpgradeFailed.selector);
        upgradeProxy.handle(origin, message);
    }
}