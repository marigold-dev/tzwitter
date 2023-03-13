import { useEffect, useState } from 'react';
import { Tweet } from '../lib/tweet';
import { Tzwitter } from '../lib/tzwitter';
import NumberOfTweets from '../components/NumberOfTweets';
import Feed from '../components/Feed';

type FeedKind = 'owned' | 'written' | 'all';

interface FeedProperty {
  tzwitter: Tzwitter;
  publicKeyHash?: string;
  onTweetClick?: (tweetId: number) => () => void;
  onAuthorClick?: (author: string) => () => void;
  feedKind: FeedKind;
}

const FeedContainer = ({
  tzwitter,
  publicKeyHash,
  onTweetClick,
  onAuthorClick,
  feedKind,
}: FeedProperty) => {
  const [tweets, setTweets] = useState<Array<Tweet>>([]);

  useEffect(() => {
    console.log(tzwitter.getOwnedTweets);

    const getTweets = () =>
      feedKind === 'owned' && publicKeyHash
        ? tzwitter.getOwnedTweets(publicKeyHash)
        : feedKind === 'written' && publicKeyHash
        ? tzwitter.getWrittenTweets(publicKeyHash)
        : tzwitter.getTweets();

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

  const onLike = (tweetId: number) => async () => {
    return await tzwitter.like(tweetId);
  };

  return (
    <>
      <NumberOfTweets number={tweets.length} />
      <Feed
        tweets={tweets}
        onLike={onLike}
        onAuthorClick={onAuthorClick}
        onTweetClick={onTweetClick}
      />
    </>
  );
};

export default FeedContainer;
