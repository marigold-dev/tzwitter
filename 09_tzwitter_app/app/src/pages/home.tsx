import { useEffect, useState } from 'react';
import './css/index.css';
import { InMemorySigner } from '@taquito/signer';
import { Tzwitter } from '../lib/tzwitter';
import { Tweet } from '../lib/tweet';
import NumberOfTweets from '../components/NumberOfTweets';
import Feed from '../components/Feed';
import Input from '../components/Input';

const secret = 'edsk3a5SDDdMWw3Q5hPiJwDXUosmZMTuKQkriPqY6UqtSfdLifpZbB';
const signer = new InMemorySigner(secret);
const TEZOS_URL = 'http://localhost:18731';
const ROLLUP_URL = 'http://localhost:8932';

const tzwitter = new Tzwitter({
  signer,
  tezosUrl: TEZOS_URL,
  rollupUrl: ROLLUP_URL,
  verbose: true,
});

const Home = () => {
  const [tweet, setTweet] = useState('');
  const [tweets, setTweets] = useState<Array<Tweet>>([]);

  useEffect(() => {
    const retrieveTweets = async () => {
      const tzwIds = await tzwitter.getTweets();
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
  }, []);

  const post = async () => {
    await tzwitter.postTweet(tweet);
    setTweet('');
  };

  const onLike = (tweetId: number) => async () => {
    return await tzwitter.like(tweetId);
  };

  return (
    <div id="container">
      <div id="content">
        <Input
          value={tweet}
          onChange={(evt) => setTweet(evt.target.value)}
          onSubmit={post}
          disabled={!tweet}
        />
        <NumberOfTweets number={tweets.length} />
        <Feed tweets={tweets} onLike={onLike} />
      </div>
    </div>
  );
};

export default Home;
