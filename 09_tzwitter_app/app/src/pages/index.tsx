import { useEffect, useState } from 'react';
import './css/index.css';
import { InMemorySigner } from '@taquito/signer';
import { Tzwitter } from '../lib/tzwitter';

const secret = 'edsk3a5SDDdMWw3Q5hPiJwDXUosmZMTuKQkriPqY6UqtSfdLifpZbB';
const signer = new InMemorySigner(secret);
const TEZOS_URL = 'https://rpc.mondaynet-2023-03-06.teztnets.xyz';
const ROLLUP_URL = 'http://localhost:8932';

const tzwitter = new Tzwitter({
  signer,
  tezosUrl: TEZOS_URL,
  rollupUrl: ROLLUP_URL,
  verbose: true,
});

const Index = () => {
  const [tweet, setTweet] = useState('');
  const [tweets, setTweets] = useState<Array<string>>([]);

  useEffect(() => {
    const id = setInterval(async () => {
      const tzwIds = await tzwitter.getTweets();
      const tweets = await Promise.all(
        tzwIds.map((id) => {
          return tzwitter.getTweet(id);
        }),
      );
      setTweets(tweets);
    }, 5000);
    return () => {
      clearInterval(id);
    };
  }, []);

  const post = async () => {
    await tzwitter.postTweet(tweet);
    setTweet('');
  };

  return (
    <div className="App">
      <header className="App-header">
        <p> Welcome to Tzwitter</p>
        <input onChange={(evt) => setTweet(evt.target.value)} value={tweet} />
        <button onClick={post}>Send the tweet</button>

        {tweets.map((tweet) => (
          <div key={tweet}>{tweet}</div>
        ))}
      </header>
    </div>
  );
};

export default Index;
