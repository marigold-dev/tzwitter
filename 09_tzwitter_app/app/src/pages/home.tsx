import { useState } from 'react';
import './css/index.css';
import { Tzwitter } from '../lib/tzwitter';
import Input from '../components/Input';
import FeedContainer from '../containers/Feed';
import { useNavigate } from 'react-router-dom';
import Layout from '../containers/Layout';
import { MenuEntries } from '../components/menu';

interface HomeProperty {
  tzwitter: Tzwitter;
  menu: MenuEntries;
}

const Home = ({ tzwitter, menu }: HomeProperty) => {
  const [tweet, setTweet] = useState('');
  const navigate = useNavigate();

  const post = async () => {
    await tzwitter.postTweet(tweet);
    setTweet('');
  };

  const onAuthorClick = (author: string) => () => {
    navigate(`/feed/${author}`);
  };

  const onLike = (tweetId: number) => () => {
    tzwitter.like(tweetId);
    return;
  };

  return (
    <Layout menu={menu} current="home">
      <Input
        value={tweet}
        onChange={(evt) => setTweet(evt.target.value)}
        onSubmit={post}
        disabled={!tweet}
      />
      <FeedContainer
        tzwitter={tzwitter}
        onLike={onLike}
        onAuthorClick={onAuthorClick}
        feedKind="all"
      />
    </Layout>
  );
};

export default Home;
