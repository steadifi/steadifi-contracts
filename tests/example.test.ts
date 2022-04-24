import { Int } from '@terra-money/terra.js';
import {
  queryNativeTokenBalance, queryTokenBalance, executeContract, sendNativeTokens, sendCW20Tokens,
} from './utils/utils';
import Context from './utils/context';

it('sends transaction', async () => {
  const ctx = Context.instance();
  const sender = ctx.getTestWallet('test1');
  const receiver = ctx.getTestWallet('test2');

  const balanceSenderBefore = new Int(await queryNativeTokenBalance(ctx.client, sender, 'uluna'));
  const balanceReceiverBefore = new Int(await queryNativeTokenBalance(ctx.client, receiver, 'uluna'));
  const tx = await sendNativeTokens(ctx.client, sender, receiver, '1000000uluna');
  const balanceSenderAfter = new Int(await queryNativeTokenBalance(ctx.client, sender, 'uluna'));
  const balanceReceiverAfter = new Int(await queryNativeTokenBalance(ctx.client, receiver, 'uluna'));

  expect(balanceSenderAfter).toEqual(balanceSenderBefore.sub(1000000).sub(4500000));
  expect(balanceReceiverAfter).toEqual(balanceReceiverBefore.add(1000000));

  // it's possible to also access transaction attributes
  const senderAddress = tx.getAttributeValue('transfer', 'sender')[0];
  expect(sender.key.accAddress).toEqual(senderAddress);
});

it('gets deployer and creator of two contracts', async () => {
  const ctx = Context.instance();
  const wallet = ctx.getTestWallet('test1');
  const { codeId } = ctx.getCodeInfo('collateral_manager');
  const deployer = (await ctx.client.wasm.codeInfo(codeId)).creator;
  expect(wallet.key.accAddress).toEqual(deployer);

  const tokenAddress = ctx.getContractInfo('cw20_base_ANDR').contractAddress;
  const response = await ctx.client.wasm.contractInfo(tokenAddress);

  expect(response.init_msg.symbol).toEqual('ANDR');
  expect(response.creator).toEqual(wallet.key.accAddress);
});

it('queries CW20 balance', async () => {
  const ctx = Context.instance();
  const user = ctx.getTestWallet('test2');

  const andrToken = ctx.getContractInfo('cw20_base_ANDR').contractAddress;

  const balance = await queryTokenBalance(ctx.client, user, andrToken);
  expect(balance).toEqual('1000');
});

it('sends CW20 tokens', async () => {
  const ctx = Context.instance();
  const sender = ctx.getTestWallet('test2');
  const receiver = ctx.getTestWallet('test1');
  const tokenAddr = ctx.getContractInfo('cw20_base_ANDR').contractAddress;

  const balanceSenderBefore = new Int(await queryTokenBalance(ctx.client, sender, tokenAddr));
  const balanceReceiverBefore = new Int(await queryTokenBalance(ctx.client, receiver, tokenAddr));
  const tx = await sendCW20Tokens(ctx.client, sender, receiver, '100', tokenAddr);
  const balanceSenderAfter = new Int(await queryTokenBalance(ctx.client, sender, tokenAddr));
  const balanceReceiverAfter = new Int(await queryTokenBalance(ctx.client, receiver, tokenAddr));

  expect(balanceSenderAfter).toEqual(balanceSenderBefore.sub(100));
  expect(balanceReceiverAfter).toEqual(balanceReceiverBefore.add(100));
  expect(tx.getAttributeValue('wasm', 'amount')[0]).toEqual('100');
});

it('mints CW20 tokens', async () => {
  const ctx = Context.instance();
  const minter = ctx.getTestWallet('test1');
  const receiver = ctx.getTestWallet('test2');
  const tokenAddr = ctx.getContractInfo('cw20_base_ANDR').contractAddress;

  const balanceReceiverBefore = new Int(await queryTokenBalance(ctx.client, receiver, tokenAddr));
  await executeContract(ctx.client, minter, tokenAddr, {
    mint: {
      amount: '100',
      recipient: receiver.key.accAddress,
    },
  });
  const balanceReceiverAfter = new Int(await queryTokenBalance(ctx.client, receiver, tokenAddr));

  expect(balanceReceiverAfter).toEqual(balanceReceiverBefore.add(100));
});

it('fails to mint CW20 tokens', async () => {
  const ctx = Context.instance();
  const wrongMinter = ctx.getTestWallet('test3');
  const receiver = ctx.getTestWallet('test2');
  const tokenAddr = ctx.getContractInfo('cw20_base_ANDR').contractAddress;

  const balanceReceiverBefore = new Int(await queryTokenBalance(ctx.client, receiver, tokenAddr));
  const promise = executeContract(ctx.client, wrongMinter, tokenAddr, {
    mint: {
      amount: '100',
      recipient: receiver.key.accAddress,
    },
  });
  // IMPORTANT: expect(promise) needs to have an *await*, otherwise the test will always succeed!
  await expect(promise).rejects.toThrow();
  const balanceReceiverAfter = new Int(await queryTokenBalance(ctx.client, receiver, tokenAddr));

  expect(balanceReceiverAfter).toEqual(balanceReceiverBefore);
});

// TODO: Check wrong wallet cannot migrate
