import './css/index.css';
import FeedContainer from '../containers/Feed';
import { useNavigate, useParams } from 'react-router-dom';
import FeedHeader from '../components/FeedHeader';
import Menu from '../components/menu';
import { Tzwitter } from '../lib/tzwitter';
import FeedKind from '../components/FeedKind';
import { useState } from 'react';

interface FeedProperty {
  tzwitter: Tzwitter;
}

const Feed = ({ tzwitter }: FeedProperty) => {
  const { publicKeyHash } = useParams() as { publicKeyHash: string };
  const [feedKind, setFeedKind] = useState<'owned' | 'written'>('written');
  const navigate = useNavigate();

  const goToHome = () => {
    navigate('/');
  };

  return (
    <div id="container">
      <Menu current={`/feed/${publicKeyHash}`} navigate={navigate} />
      <div id="content">
        <FeedHeader author={publicKeyHash} goToHome={goToHome} />
        <FeedKind value={feedKind} onChange={setFeedKind} />
        <FeedContainer
          publicKeyHash={publicKeyHash}
          tzwitter={tzwitter}
          feedKind={feedKind}
        />
      </div>
    </div>
  );
};

export default Feed;
