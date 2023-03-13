import { useEffect, useState } from 'react';
import './css/index.css';
import { InMemorySigner } from '@taquito/signer';
import { Tzwitter } from '../lib/tzwitter';
import Input from '../components/Input';
import FeedContainer from '../containers/Feed';
import Menu from '../components/menu';
import { useNavigate } from 'react-router-dom';

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
  const navigate = useNavigate();

  const [address, setAddress] = useState('');
  useEffect(() => {
    signer.publicKeyHash().then(setAddress);
  }, []);

  const post = async () => {
    await tzwitter.postTweet(tweet);
    setTweet('');
  };

  return (
    <div id="container">
      <Menu current={'/'} navigate={navigate} address={address} />
      <div id="content">
        <Input
          value={tweet}
          onChange={(evt) => setTweet(evt.target.value)}
          onSubmit={post}
          disabled={!tweet}
        />
        <FeedContainer tzwitter={tzwitter} />
      </div>
    </div>
  );
};

export default Home;
