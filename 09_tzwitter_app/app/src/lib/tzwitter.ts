import { blake2bHex } from 'blakejs';
import { RollupClient, Signer } from './rollup';
import { Tweet } from './tweet';

class Tzwitter {
  private signer: Signer;
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
    this.signer = signer;
    this.rollupClient = new RollupClient({
      signer,
      tezosUrl,
      rollupUrl,
      verbose,
    });
    this.magicByte = '74' || magicByte;
  }

  /**
   * Post a tweet to the rollup
   * @param tweet
   * @returns
   */
  async postTweet(tweet: string): Promise<string> {
    const publicKeyHash = await this.signer.publicKeyHash();
    // Compute the next nonce of the user
    const nonceBytes = await this.rollupClient.getState(
      `/accounts/${publicKeyHash}/nonce`,
    );
    const nonce = Number.parseInt(nonceBytes || '00000000', 16) + 1;

    // Hash the payload to sign it later
    const strNonce = nonce.toString(16).padStart(8, '0');
    const publicKey = await this.signer.publicKey();
    const toHash = `${strNonce}${publicKeyHash}${tweet}`;
    console.log(toHash);
    const hash = blake2bHex(toHash, undefined, 32);

    // Sign the payload
    const { prefixSig } = await this.signer.sign(hash);
    // Construct the request
    const request = {
      pkey: {
        Ed25519: publicKey,
      },
      signature: {
        Ed25519: prefixSig,
      },
      inner: {
        nonce: nonce,
        content: {
          PostTweet: {
            author: {
              Tz1: publicKeyHash,
            },
            content: tweet,
          },
        },
      },
    };
    const strRequest = JSON.stringify(request);
    const payload = Buffer.from(strRequest).toString('hex');
    console.log(payload);
    // Add the magic byte and send the payload
    return this.rollupClient.send(this.magicByte + payload);
  }

  /**
   * Retrieve a tweet of a given id
   * @param tweetId the id of the tweet
   * @returns the Tweet as a promise
   */
  async getTweet(tweetId: number): Promise<Tweet> {
    const authorPath = `/tweets/${tweetId}/author`;
    const contentPath = `/tweets/${tweetId}/content`;
    const likesPath = `/tweets/${tweetId}/likes`;

    const authorBytes = await this.rollupClient.getState(authorPath);
    const contentBytes = await this.rollupClient.getState(contentPath);
    const likesBytes = await this.rollupClient.getState(likesPath);

    const author = Buffer.from(authorBytes, 'hex').toString('utf-8');
    const content = Buffer.from(contentBytes, 'hex').toString('utf-8');
    const likes = Number('0x' + likesBytes);

    return { id: tweetId, author, content, likes };
  }

  /**
   * Retrieves all the tweets
   * @returns
   */
  async getTweets(): Promise<Array<number>> {
    const path = '/tweets';
    const ids = await this.rollupClient.getSubkeys(path);
    return ids
      .map((id: string) => Number.parseInt(id))
      .sort()
      .reverse();
  }

  /**
   * Get tweets owned by the given public key
   * @param publicKeyHash
   * @returns the id list of owned tweets
   */
  async getOwnedTweets(publicKeyHash: string): Promise<Array<number>> {
    const path = publicKeyHash
      ? `/accounts/${publicKeyHash}/tweets/owned`
      : '/tweets';
    const ids = await this.rollupClient.getSubkeys(path);
    return ids
      .map((id: string) => Number.parseInt(id))
      .sort()
      .reverse();
  }

  /**
   * Get tweets written by the given public key
   * @param publicKeyHash
   * @returns the id list of written tweets
   */
  async getWrittenTweets(publicKeyHash: string): Promise<Array<number>> {
    const path = publicKeyHash
      ? `/accounts/${publicKeyHash}/tweets/written`
      : '/tweets';
    const ids = await this.rollupClient.getSubkeys(path);
    return ids
      .map((id: string) => Number.parseInt(id))
      .sort()
      .reverse();
  }

  /**
   * Like a tweet, it will increment by one the number like of this tweet
   * @param tweetId the id of the tweet to like
   * @returns the hash of the operation
   */
  async like(tweetId: number): Promise<string> {
    const publicKeyHash = await this.signer.publicKeyHash();
    // Compute the next nonce of the user
    const nonceBytes = await this.rollupClient.getState(
      `/accounts/${publicKeyHash}/nonce`,
    );
    const nonce = Number.parseInt(nonceBytes || '00000000', 16) + 1;

    // Hash the payload to sign it later
    const strNonce = nonce.toString(16).padStart(8, '0');
    const publicKey = await this.signer.publicKey();
    const toHash = `${strNonce}${tweetId}`;
    const hash = blake2bHex(toHash, undefined, 32);

    // Sign the payload
    const { prefixSig } = await this.signer.sign(hash);

    // Construct the request
    const request = {
      pkey: {
        Ed25519: publicKey,
      },
      signature: {
        Ed25519: prefixSig,
      },
      inner: {
        nonce: nonce,
        content: {
          LikeTweet: tweetId,
        },
      },
    };
    const strRequest = JSON.stringify(request);
    const payload = Buffer.from(strRequest).toString('hex');
    return this.rollupClient.send(this.magicByte + payload);
  }

  async transferTweet(tweetId: number, destination: string): Promise<string> {
    const publicKeyHash = await this.signer.publicKeyHash();
    // Compute the next nonce of the user
    const nonceBytes = await this.rollupClient.getState(
      `/accounts/${publicKeyHash}/nonce`,
    );
    const nonce = Number.parseInt(nonceBytes || '00000000', 16) + 1;

    // Hash the payload to sign it later
    const strNonce = nonce.toString(16).padStart(8, '0');
    const publicKey = await this.signer.publicKey();
    const toHash = `${strNonce}${destination}${tweetId}`;
    const hash = blake2bHex(toHash, undefined, 32);

    // Sign the payload
    const { prefixSig } = await this.signer.sign(hash);
    // Construct the request
    const request = {
      pkey: {
        Ed25519: publicKey,
      },
      signature: {
        Ed25519: prefixSig,
      },
      inner: {
        nonce: nonce,
        content: {
          Transfer: {
            destination: {
              Tz1: destination,
            },
            tweet_id: tweetId,
          },
        },
      },
    };
    const strRequest = JSON.stringify(request);
    const payload = Buffer.from(strRequest).toString('hex');
    console.log(payload);
    // Add the magic byte and send the payload
    return this.rollupClient.send(this.magicByte + payload);
  }
}

export { Tzwitter };
