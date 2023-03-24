import './css/index.css';
import FeedContainer from '../containers/Feed';
import { useNavigate, useParams } from 'react-router-dom';
import FeedHeader from '../components/FeedHeader';
import { Tzwitter } from '../lib/tzwitter';
import FeedKind from '../components/FeedKind';
import { useState } from 'react';
import Layout from '../containers/Layout';
import { MenuEntries } from '../components/menu';

interface FeedProperty {
  tzwitter: Tzwitter;
  menu: MenuEntries;
}

const Feed = ({ tzwitter, menu }: FeedProperty) => {
  const { publicKeyHash } = useParams() as { publicKeyHash: string };
  const [feedKind, setFeedKind] = useState<'owned' | 'written'>('written');
  const navigate = useNavigate();

  const goToHome = () => {
    navigate('/');
  };

  const onLike = (tweetId: number) => () => {
    tzwitter.like(tweetId);
    return;
  };

  return (
    <Layout menu={menu} current="feed">
      <FeedHeader author={publicKeyHash} goToHome={goToHome} />
      <FeedKind value={feedKind} onChange={setFeedKind} />
      <FeedContainer
        publicKeyHash={publicKeyHash}
        tzwitter={tzwitter}
        onLike={onLike}
        feedKind={feedKind}
      />
    </Layout>
  );
};

export default Feed;
