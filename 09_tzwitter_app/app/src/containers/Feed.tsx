import { useEffect, useState } from 'react';
import { Tweet } from '../lib/tweet';
import { Tzwitter } from '../lib/tzwitter';
import NumberOfTweets from '../components/NumberOfTweets';
import Feed from '../components/Feed';

type FeedKind = 'owned' | 'written' | 'collecting' | 'all';

interface FeedProperty {
  tzwitter: Tzwitter;
  publicKeyHash?: string;
  onTransfer?: (tweetId: number) => () => void;
  onAuthorClick?: (author: string) => () => void;
  onLike?: (tweetId: number) => () => void;
  feedKind: FeedKind;
  onCollect?: (tweetId: number) => () => void;
}

const FeedContainer = ({
  tzwitter,
  publicKeyHash,
  onTransfer,
  onAuthorClick,
  onLike,
  feedKind,
  onCollect,
}: FeedProperty) => {
  const [tweets, setTweets] = useState<Array<Tweet>>([]);

  useEffect(() => {
    const getTweets = async (): Promise<Array<number>> => {
      switch (feedKind) {
        case 'owned':
          return publicKeyHash ? tzwitter.getOwnedTweets(publicKeyHash) : [];
        case 'written':
          return publicKeyHash ? tzwitter.getWrittenTweets(publicKeyHash) : [];
        case 'collecting': {
          return publicKeyHash
            ? tzwitter.getCollectedTweets(publicKeyHash)
            : [];
        }
        case 'all':
        default:
          return tzwitter.getTweets();
      }
    };

    const retrieveTweets = async () => {
      const tzwIds = await getTweets();
      const tweets = await Promise.all(
        tzwIds.map((id) => {
          return tzwitter.getTweet(id);
        }),
      );
      setTweets(tweets);
    };
    retrieveTweets();
    const id = setInterval(retrieveTweets, 5000);
    return () => {
      clearInterval(id);
    };
  }, [feedKind, publicKeyHash, tzwitter]);

  return (
    <>
      <NumberOfTweets number={tweets.length} />
      <Feed
        tweets={tweets}
        onLike={onLike}
        onAuthorClick={onAuthorClick}
        onTransfer={onTransfer}
        onCollect={onCollect}
      />
    </>
  );
};

export default FeedContainer;
