import './css/index.css';
import { InMemorySigner } from '@taquito/signer';
import { Tzwitter } from '../lib/tzwitter';
import FeedContainer from '../containers/Feed';
import { useNavigate, useParams } from 'react-router-dom';
import FeedHeader from '../components/FeedHeader';

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

const Account = () => {
  const { publicKeyHash } = useParams() as { publicKeyHash: string };
  const navigate = useNavigate();

  const goToHome = () => {
    navigate('/');
  };

  return (
    <div id="container">
      <div id="content">
        <FeedHeader author={publicKeyHash} goToHome={goToHome} />
        <FeedContainer publicKeyHash={publicKeyHash} tzwitter={tzwitter} />
      </div>
    </div>
  );
};

export default Account;
