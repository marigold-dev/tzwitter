import { useEffect, useState } from 'react';
import { Tweet } from '../lib/tweet';
import { Tzwitter } from '../lib/tzwitter';
import NumberOfTweets from '../components/NumberOfTweets';
import Feed from '../components/Feed';
import { useNavigate } from 'react-router-dom';

interface FeedProperty {
  tzwitter: Tzwitter;
  publicKeyHash?: string;
  onTweetClick?: (tweetId: number) => () => void;
}

const FeedContainer = ({
  tzwitter,
  publicKeyHash,
  onTweetClick,
}: FeedProperty) => {
  const [tweets, setTweets] = useState<Array<Tweet>>([]);
  const navigate = useNavigate();

  useEffect(() => {
    const retrieveTweets = async () => {
      const tzwIds = await tzwitter.getTweets(publicKeyHash);
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
  }, [publicKeyHash, tzwitter]);

  const onLike = (tweetId: number) => async () => {
    return await tzwitter.like(tweetId);
  };

  const onAuthorClick = (author: string) => () => {
    navigate(`/feed/${author}`);
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
