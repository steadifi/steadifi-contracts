import { MsgSend } from '@terra-money/terra.js';
import { sendTransaction, queryNativeTokenBalance } from './utils/utils';
import Context from './utils/context';

it('works', async () => {
  const ctx = Context.instance();
  const sender = ctx.getTestWallet('test1');
  const receiver = ctx.getTestWallet('test2');

  const msg = new MsgSend(sender.key.accAddress, receiver.key.accAddress, '1000000uluna');
  console.log(await queryNativeTokenBalance(ctx.client, sender, 'uluna'));
  console.log(await queryNativeTokenBalance(ctx.client, receiver, 'uluna'));
  await sendTransaction(ctx.client, sender, msg);
  console.log(await queryNativeTokenBalance(ctx.client, sender, 'uluna'));
  console.log(await queryNativeTokenBalance(ctx.client, receiver, 'uluna'));
});

it('works2', () => {
  Context.instance();
  Context.instance();
});
