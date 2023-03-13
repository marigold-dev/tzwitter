import './css/index.css';
import FeedContainer from '../containers/Feed';
import { useNavigate, useParams } from 'react-router-dom';
import FeedHeader from '../components/FeedHeader';
import Menu from '../components/menu';
import { Tzwitter } from '../lib/tzwitter';

interface FeedProperty {
  tzwitter: Tzwitter;
}

const Feed = ({ tzwitter }: FeedProperty) => {
  const { publicKeyHash } = useParams() as { publicKeyHash: string };
  const navigate = useNavigate();

  const goToHome = () => {
    navigate('/');
  };

  return (
    <div id="container">
      <Menu current={`/feed/${publicKeyHash}`} navigate={navigate} />
      <div id="content">
        <FeedHeader author={publicKeyHash} goToHome={goToHome} />
        <FeedContainer publicKeyHash={publicKeyHash} tzwitter={tzwitter} />
      </div>
    </div>
  );
};

export default Feed;
