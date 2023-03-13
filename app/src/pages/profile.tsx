import { useNavigate } from 'react-router-dom';
import Menu from '../components/menu';
import ProfileHeader from '../components/ProfileHeader';
import { InMemorySigner } from '@taquito/signer';
import FeedContainer from '../containers/Feed';
import { Tzwitter } from '../lib/tzwitter';
import { useEffect, useState } from 'react';

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

const Profile = () => {
  const navigate = useNavigate();

  const [address, setAddress] = useState('');
  useEffect(() => {
    signer.publicKeyHash().then(setAddress);
  }, []);

  return (
    <div id="container">
      <Menu current={'/profile'} navigate={navigate} />
      <div id="content">
        <ProfileHeader />
        <FeedContainer publicKeyHash={address} tzwitter={tzwitter} />
      </div>
    </div>
  );
};

export default Profile;
