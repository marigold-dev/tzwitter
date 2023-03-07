import { RollupClient, Signer } from './rollup';

class Tzwitter {
  private rollupClient: RollupClient;
  private magicByte: string;

  constructor({
    signer,
    tezosUrl,
    rollupUrl,
    magicByte,
    verbose,
  }: {
    signer: Signer;
    tezosUrl: string;
    rollupUrl: string;
    magicByte?: string;
    verbose?: boolean;
  }) {
    this.rollupClient = new RollupClient({
      signer,
      tezosUrl,
      rollupUrl,
      verbose,
    });
    this.magicByte = '74' || magicByte;
  }

  async postTweet(tweet: string): Promise<string> {
    const bytes = Buffer.from(tweet).toString('hex');
    const payload = this.magicByte + bytes;
    const op = await this.rollupClient.send(payload);
    return op;
  }

  async getTweet(tweetId: string): Promise<string> {
    const path = `/tweets/${tweetId}`;
    const bytes = await this.rollupClient.getState(path);
    const tweet = Buffer.from(bytes, 'hex').toString('utf-8');
    return tweet;
  }

  async getTweets(): Promise<Array<string>> {
    const path = `/tweets`;
    const ids = await this.rollupClient.getSubkeys(path);
    return ids;
  }
}

export { Tzwitter };
