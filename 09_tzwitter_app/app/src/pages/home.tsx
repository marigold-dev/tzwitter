import { useState } from 'react';
import './css/index.css';
import { Tzwitter } from '../lib/tzwitter';
import Input from '../components/Input';
import FeedContainer from '../containers/Feed';
import Menu from '../components/menu';
import { useNavigate } from 'react-router-dom';

interface HomeProperty {
  tzwitter: Tzwitter;
}

const Home = ({ tzwitter }: HomeProperty) => {
  const [tweet, setTweet] = useState('');
  const navigate = useNavigate();

  const post = async () => {
    await tzwitter.postTweet(tweet);
    setTweet('');
  };

  const onAuthorClick = (author: string) => () => {
    navigate(`/feed/${author}`);
  };

  return (
    <div id="container">
      <Menu current={'/'} navigate={navigate} />
      <div id="content">
        <Input
          value={tweet}
          onChange={(evt) => setTweet(evt.target.value)}
          onSubmit={post}
          disabled={!tweet}
        />
        <FeedContainer tzwitter={tzwitter} onAuthorClick={onAuthorClick} />
      </div>
    </div>
  );
};

export default Home;
