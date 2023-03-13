import { useNavigate } from 'react-router-dom';
import Menu from '../components/menu';
import ProfileHeader from '../components/ProfileHeader';
import FeedContainer from '../containers/Feed';
import { Tzwitter } from '../lib/tzwitter';
import Account from '../lib/account';

interface ProfileProperty {
  tzwitter: Tzwitter;
  account: Account;
}

const Profile = ({ tzwitter, account }: ProfileProperty) => {
  const navigate = useNavigate();

  const onTweetClick = (tweetId: number) => () => {
    console.log('TODO: open popup');
  };

  return (
    <div id="container">
      <Menu current={'/profile'} navigate={navigate} />
      <div id="content">
        <ProfileHeader />
        <FeedContainer
          publicKeyHash={account.publicKeyHash}
          tzwitter={tzwitter}
          onTweetClick={onTweetClick}
        />
      </div>
    </div>
  );
};

export default Profile;
