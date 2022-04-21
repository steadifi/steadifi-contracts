import { MsgSend, Int } from '@terra-money/terra.js';
import { sendTransaction, queryNativeTokenBalance } from './utils/utils';
import Context from './utils/context';

it('sends transaction', async () => {
  const ctx = Context.instance();
  const sender = ctx.getTestWallet('test1');
  const receiver = ctx.getTestWallet('test2');

  const msg = new MsgSend(sender.key.accAddress, receiver.key.accAddress, '1000000uluna');
  const balanceSenderBefore = new Int(await queryNativeTokenBalance(ctx.client, sender, 'uluna'));
  const balanceReceiverBefore = new Int(await queryNativeTokenBalance(ctx.client, receiver, 'uluna'));
  await sendTransaction(ctx.client, sender, msg);
  const balanceSenderAfter = new Int(await queryNativeTokenBalance(ctx.client, sender, 'uluna'));
  const balanceReceiverAfter = new Int(await queryNativeTokenBalance(ctx.client, receiver, 'uluna'));

  expect(balanceSenderAfter).toEqual(balanceSenderBefore.sub(1000000).sub(4500000));
  expect(balanceReceiverAfter).toEqual(balanceReceiverBefore.add(1000000));
});

it('gets deployer of contract', async () => {
  const ctx = Context.instance();
  const wallet = ctx.getTestWallet('test1');
  const codeId = parseInt(ctx.getCodeInfo('collateral_manager').codeId, 10);
  const deployer = (await ctx.client.wasm.codeInfo(codeId)).creator;
  expect(wallet.key.accAddress).toEqual(deployer);
});
