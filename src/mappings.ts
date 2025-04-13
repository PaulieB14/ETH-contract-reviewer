import { Contract, Wallet, Interaction, DailyContractStat, DailyStat } from '../generated/schema';
import { BigInt, BigDecimal, Address, ethereum } from '@graphprotocol/graph-ts';

export function handleContractInteraction(
  contractAddress: string,
  walletAddress: string,
  blockNumber: BigInt,
  timestamp: BigInt
): void {
  // Get or create contract entity
  let contract = Contract.load(contractAddress);
  if (!contract) {
    contract = new Contract(contractAddress);
    contract.address = contractAddress;
    contract.firstInteractionBlock = blockNumber;
    contract.totalCalls = BigInt.fromI32(0);
    contract.uniqueWallets = BigInt.fromI32(0);
    contract.avgCallsPerWallet = BigDecimal.fromString('0');
    contract.isNewContract = true;
  }
  
  contract.lastInteractionBlock = blockNumber;
  contract.totalCalls = contract.totalCalls.plus(BigInt.fromI32(1));
  
  // Get or create wallet entity
  let wallet = Wallet.load(walletAddress);
  if (!wallet) {
    wallet = new Wallet(walletAddress);
    wallet.address = walletAddress;
    wallet.contractsInteracted = 0;
    wallet.totalInteractions = 0;
    
    // Increment unique wallets for this contract
    contract.uniqueWallets = contract.uniqueWallets.plus(BigInt.fromI32(1));
  }
  
  wallet.totalInteractions += 1;
  
  // Create interaction entity
  const interactionId = `${contractAddress}-${walletAddress}-${blockNumber.toString()}`;
  let interaction = new Interaction(interactionId);
  interaction.contract = contractAddress;
  interaction.wallet = walletAddress;
  interaction.blockNumber = blockNumber;
  interaction.timestamp = timestamp;
  
  // Update average calls per wallet
  if (contract.uniqueWallets.gt(BigInt.fromI32(0))) {
    contract.avgCallsPerWallet = contract.totalCalls.toBigDecimal().div(contract.uniqueWallets.toBigDecimal());
  }
  
  // Handle daily stats
  const dayTimestamp = timestamp.div(BigInt.fromI32(86400)).times(BigInt.fromI32(86400));
  const dailyContractStatId = `${contractAddress}-${dayTimestamp.toString()}`;
  
  let dailyContractStat = DailyContractStat.load(dailyContractStatId);
  if (!dailyContractStat) {
    dailyContractStat = new DailyContractStat(dailyContractStatId);
    dailyContractStat.contract = contractAddress;
    dailyContractStat.dayTimestamp = dayTimestamp;
    dailyContractStat.calls = BigInt.fromI32(0);
    dailyContractStat.uniqueWallets = BigInt.fromI32(0);
  }
  
  dailyContractStat.calls = dailyContractStat.calls.plus(BigInt.fromI32(1));
  
  // Update global daily stats
  const dailyStatId = dayTimestamp.toString();
  let dailyStat = DailyStat.load(dailyStatId);
  if (!dailyStat) {
    dailyStat = new DailyStat(dailyStatId);
    dailyStat.dayTimestamp = dayTimestamp;
    dailyStat.activeContracts = BigInt.fromI32(0);
    dailyStat.newContracts = BigInt.fromI32(0);
    dailyStat.totalCalls = BigInt.fromI32(0);
    dailyStat.uniqueWallets = BigInt.fromI32(0);
  }
  
  dailyStat.totalCalls = dailyStat.totalCalls.plus(BigInt.fromI32(1));
  
  if (contract.isNewContract) {
    dailyStat.newContracts = dailyStat.newContracts.plus(BigInt.fromI32(1));
    contract.isNewContract = false;
  }
  
  // Save all entities
  contract.save();
  wallet.save();
  interaction.save();
  dailyContractStat.save();
  dailyStat.save();
}
