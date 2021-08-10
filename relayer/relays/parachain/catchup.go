package parachain

import (
	"context"
	"fmt"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/sirupsen/logrus"
	"github.com/snowfork/go-substrate-rpc-client/v3/types"
	"github.com/snowfork/snowbridge/relayer/chain/parachain"
	"github.com/snowfork/snowbridge/relayer/contracts/basic"
	"github.com/snowfork/snowbridge/relayer/contracts/incentivized"

	log "github.com/sirupsen/logrus"
)

// Catches up by searching for and relaying all missed commitments before the given para block
// This method implicitly assumes that relaychainBlock or some earlier relay chain block has
// already finalized the given para block
func (li *BeefyListener) buildMissedMessagePackages(
	ctx context.Context, relaychainBlock uint64, paraBlock uint64, paraHash types.Hash) (
	[]MessagePackage, error) {
	basicContract, err := basic.NewBasicInboundChannel(common.HexToAddress(
		li.config.Contracts.BasicInboundChannel),
		li.ethereumConn.GetClient(),
	)
	if err != nil {
		return nil, err
	}

	incentivizedContract, err := incentivized.NewIncentivizedInboundChannel(common.HexToAddress(
		li.config.Contracts.IncentivizedInboundChannel),
		li.ethereumConn.GetClient(),
	)
	if err != nil {
		return nil, err
	}

	options := bind.CallOpts{
		Pending: true,
		Context: ctx,
	}

	ethBasicNonce, err := basicContract.Nonce(&options)
	if err != nil {
		return nil, err
	}
	log.WithFields(logrus.Fields{
		"nonce": ethBasicNonce,
	}).Info("Checked latest nonce delivered to ethereum basic channel")

	ethIncentivizedNonce, err := incentivizedContract.Nonce(&options)
	if err != nil {
		return nil, err
	}
	log.WithFields(logrus.Fields{
		"nonce": ethIncentivizedNonce,
	}).Info("Checked latest nonce delivered to ethereum incentivized channel")

	paraBasicNonceKey, err := types.CreateStorageKey(li.parachainConnection.Metadata(), "BasicOutboundModule", "Nonce", nil, nil)
	if err != nil {
		log.Error(err)
		return nil, err
	}
	var paraBasicNonce types.U64
	ok, err := li.parachainConnection.API().RPC.State.GetStorage(paraBasicNonceKey, &paraBasicNonce, paraHash)
	if err != nil {
		log.Error(err)
		return nil, err
	}
	if !ok {
		paraBasicNonce = 0
	}
	log.WithFields(logrus.Fields{
		"nonce": uint64(paraBasicNonce),
	}).Info("Checked latest nonce generated by parachain basic channel")

	paraIncentivizedNonceKey, err := types.CreateStorageKey(li.parachainConnection.Metadata(), "IncentivizedOutboundModule", "Nonce", nil, nil)
	if err != nil {
		log.Error(err)
		return nil, err
	}
	var paraIncentivizedNonce types.U64
	ok, err = li.parachainConnection.API().RPC.State.GetStorage(paraIncentivizedNonceKey, &paraIncentivizedNonce, paraHash)
	if err != nil {
		log.Error(err)
		return nil, err
	}
	if !ok {
		paraIncentivizedNonce = 0
	}
	log.WithFields(logrus.Fields{
		"nonce": uint64(paraIncentivizedNonce),
	}).Info("Checked latest nonce generated by parachain incentivized channel")

	if ethBasicNonce == uint64(paraBasicNonce) && ethIncentivizedNonce == uint64(paraIncentivizedNonce) {
		return nil, nil
	}

	log.Info("Nonces are not all up to date - searching for lost commitments")

	paraBlocks, err := li.searchForLostCommitments(paraBlock, ethBasicNonce, ethIncentivizedNonce)
	if err != nil {
		return nil, err
	}

	log.Info("Stopped searching for lost commitments")

	log.WithFields(logrus.Fields{
		"blocks": paraBlocks,
	}).Info("Found these blocks and commitments")

	blocksWithProofs, err := li.parablocksWithProofs(paraBlocks, relaychainBlock)
	if err != nil {
		log.Error(err)
		return nil, err
	}

	// Reverse blocks to be in ascending order
	for i, j := 0, len(blocksWithProofs)-1; i < j; i, j = i+1, j-1 {
		blocksWithProofs[i], blocksWithProofs[j] = blocksWithProofs[j], blocksWithProofs[i]
	}

	log.WithFields(logrus.Fields{
		"blocks": blocksWithProofs,
	}).Info("Packaging these blocks and proofs")

	messagePackages, err := CreateMessagePackages(blocksWithProofs)
	if err != nil {
		log.WithError(err).Error("Failed to create message packages")
		return nil, err
	}

	log.Info("Created message packages")

	return messagePackages, nil
}

// Takes a slice of parachain blocks and augments them with their respective
// header, header proof and MMR proof
func (li *BeefyListener) parablocksWithProofs(blocks []ParaBlockWithDigest, latestRelayChainBlockNumber uint64) (
	[]ParaBlockWithProofs, error) {
	relayChainBlockNumber := latestRelayChainBlockNumber
	var relayBlockHash types.Hash
	var err error
	var blocksWithProof []ParaBlockWithProofs
	for _, block := range blocks {
		var allParaHeads []types.Bytes
		var ownParaHead types.Header
		var ownParaHeadLeafIndex int

		// Loop back over relay chain blocks to find the one that finalized the given parachain block
		for ownParaHead.Number != types.BlockNumber(block.BlockNumber) {
			log.WithField("relayChainBlockNumber", relayChainBlockNumber).Info("Getting hash for relay chain block")
			relayBlockHash, err = li.relaychainConn.API().RPC.Chain.GetBlockHash(uint64(relayChainBlockNumber))
			if err != nil {
				log.WithError(err).Error("Failed to get block hash")
				return nil, err
			}

			log.WithField("relayBlockHash", relayBlockHash.Hex()).Info("Got relay chain blockhash")
			heads, err := li.relaychainConn.FetchParaHeads(relayBlockHash)
			if err != nil {
				log.WithError(err).Error("Failed to get paraheads")
				return nil, err
			}

			log.WithFields(logrus.Fields{
				"count": len(heads),
			}).Info("Fetched para heads")

			if _, ok := heads[li.paraID]; !ok {
				return nil, fmt.Errorf("chain is not a registered parachain")
			}

			var header types.Header
			if err := types.DecodeFromBytes(heads[li.paraID].Data, &header); err != nil {
				log.WithError(err).Error("Failed to decode Header")
				return nil, err
			}

			allParaHeads = li.relaychainConn.AsProofInput(heads)
			ownParaHead = header
			ownParaHeadLeafIndex = heads[li.paraID].LeafIndex

			relayChainBlockNumber--
		}

		log.Debug("Have inputs for proof generation")
		for i, v := range allParaHeads {
			log.WithFields(logrus.Fields{
				"LeafIndex": i,
				"HeadData": fmt.Sprintf("%#x", v),
				"IsSnowbridgePara": i == ownParaHeadLeafIndex,
			}).Debug("Head data")
		}

		// Note - relayChainBlockNumber will be one less than the block number discovered
		// due to the decrement at the end of the loop, but the mmr leaves are 0 indexed whereas
		// block numbers start from 1, so we actually do want to query for the leaf at
		// one less than the block number
		mmrProof, err := li.relaychainConn.GetMMRLeafForBlock(uint64(relayChainBlockNumber), relayBlockHash)
		if err != nil {
			log.WithError(err).Error("Failed to get mmr leaf")
			return nil, err
		}
		parasRoot, ownParaHeadProof, err := parachain.CreateParachainHeaderProof(allParaHeads, ownParaHead, mmrProof.Leaf.ParachainHeads)
		if err != nil {
			log.WithError(err).Error("Failed to create parachain header proof")
			return nil, err
		}

		blockWithProof := ParaBlockWithProofs{
			Block:            block,
			MMRProofResponse: mmrProof,
			Header:           ownParaHead,
			HeaderProof:      ownParaHeadProof,
			HeaderProofPos:   ownParaHeadLeafIndex,
			HeaderProofWidth: len(allParaHeads),
			HeaderProofRoot:  parasRoot,
		}
		blocksWithProof = append(blocksWithProof, blockWithProof)
	}
	return blocksWithProof, nil
}

// Searches for all lost commitments on each channel from the given parachain block number backwards
// until it finds the given basic and incentivized nonce
func (li *BeefyListener) searchForLostCommitments(
	lastParaBlockNumber uint64,
	basicNonceToFind uint64,
	incentivizedNonceToFind uint64) ([]ParaBlockWithDigest, error) {
	log.WithFields(logrus.Fields{
		"basicNonce":        basicNonceToFind,
		"incentivizedNonce": incentivizedNonceToFind,
		"latestblockNumber": lastParaBlockNumber,
	}).Debug("Searching backwards from latest block on parachain to find block with nonce")

	currentBlockNumber := lastParaBlockNumber + 1
	basicNonceFound := false
	incentivizedNonceFound := false
	var blocks []ParaBlockWithDigest
	for (basicNonceFound == false || incentivizedNonceFound == false) && currentBlockNumber != 0 {
		currentBlockNumber--
		log.WithFields(logrus.Fields{
			"blockNumber": currentBlockNumber,
		}).Debug("Checking header...")

		blockHash, err := li.parachainConnection.API().RPC.Chain.GetBlockHash(currentBlockNumber)
		if err != nil {
			log.WithFields(logrus.Fields{
				"blockNumber": currentBlockNumber,
			}).WithError(err).Error("Failed to fetch blockhash")
			return nil, err
		}

		header, err := li.parachainConnection.API().RPC.Chain.GetHeader(blockHash)
		if err != nil {
			log.WithError(err).Error("Failed to fetch header")
			return nil, err
		}

		digestItems, err := parachain.ExtractAuxiliaryDigestItems(header.Digest)
		if err != nil {
			return nil, err
		}

		var digestItemsWithData []DigestItemWithData

		for _, digestItem := range digestItems {
			if digestItem.IsCommitment {
				channelID := digestItem.AsCommitment.ChannelID
				if channelID.IsBasic && !basicNonceFound {
					isRelayed, messageData, err := li.checkBasicMessageNonces(&digestItem, basicNonceToFind)
					if err != nil {
						return nil, err
					}
					if isRelayed {
						basicNonceFound = true
					} else {
						item := DigestItemWithData{digestItem, messageData}
						digestItemsWithData = append(digestItemsWithData, item)
					}
				}
				if channelID.IsIncentivized && !incentivizedNonceFound {
					isRelayed, messageData, err := li.checkIncentivizedMessageNonces(&digestItem, incentivizedNonceToFind)
					if err != nil {
						return nil, err
					}
					if isRelayed {
						incentivizedNonceFound = true
					} else {
						item := DigestItemWithData{digestItem, messageData}
						digestItemsWithData = append(digestItemsWithData, item)
					}
				}
			}
		}

		if len(digestItemsWithData) != 0 {
			block := ParaBlockWithDigest{
				BlockNumber:         currentBlockNumber,
				DigestItemsWithData: digestItemsWithData,
			}
			blocks = append(blocks, block)
		}
	}

	return blocks, nil
}

func (li *BeefyListener) checkBasicMessageNonces(
	digestItem *parachain.AuxiliaryDigestItem,
	nonceToFind uint64,
) (bool, types.StorageDataRaw, error) {
	messages, data, err := li.parachainConnection.GetBasicOutboundMessages(*digestItem)
	if err != nil {
		return false, nil, err
	}

	for _, message := range messages {
		if message.Nonce <= nonceToFind {
			return true, data, nil
		}
	}
	return false, data, nil
}

func (li *BeefyListener) checkIncentivizedMessageNonces(
	digestItem *parachain.AuxiliaryDigestItem,
	nonceToFind uint64,
) (bool, types.StorageDataRaw, error) {

	messages, data, err := li.parachainConnection.GetIncentivizedOutboundMessages(*digestItem)
	if err != nil {
		return false, nil, err
	}

	for _, message := range messages {
		if message.Nonce <= nonceToFind {
			return true, data, nil
		}
	}
	return false, data, nil
}