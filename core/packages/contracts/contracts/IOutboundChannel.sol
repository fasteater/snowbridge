// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;

interface IOutboundChannel {
    function submit(bytes calldata dest, bytes calldata payload) external payable;
}
