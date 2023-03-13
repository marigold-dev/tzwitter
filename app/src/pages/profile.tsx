import { useNavigate } from 'react-router-dom';
import Menu from '../components/menu';
import ProfileHeader from '../components/ProfileHeader';
import FeedContainer from '../containers/Feed';
import { useState } from 'react';
import { Tzwitter } from '../lib/tzwitter';
import Account from '../lib/account';
import Popup from '../components/popup';
import FeedKind from '../components/FeedKind';
import './css/profile.css';

interface ProfileProperty {
  tzwitter: Tzwitter;
  account: Account;
}

interface Form {
  tweetId: number;
  destination: string;
}

const Profile = ({ tzwitter, account }: ProfileProperty) => {
  const navigate = useNavigate();
  const [form, setForm] = useState<Form | undefined>(undefined);
  const [feedKind, setFeedKind] = useState<'owned' | 'written'>('owned');

  const isOpen = !!form;
  const onClose = () => setForm(undefined);
  const disabled = !(form && form.destination);

  const onTweetClick =
    feedKind === 'owned'
      ? (tweetId: number) => () => setForm({ tweetId, destination: '' })
      : undefined;

  const onDestinationChange = (evt: React.ChangeEvent<HTMLInputElement>) =>
    setForm((form) => {
      return form ? { ...form, destination: evt.target.value } : undefined;
    });

  const onTransfer = async () => {
    if (!form || !form.destination) return null;
    const destination = form.destination;
    const tweetId = form.tweetId;
    await tzwitter.transferTweet(tweetId, destination);
    onClose();
  };

  console.log(feedKind);

  return (
    <div id="container">
      <Menu current={'/profile'} navigate={navigate} />
      <div id="content">
        <ProfileHeader />
        <FeedKind value={feedKind} onChange={setFeedKind} />
        <FeedContainer
          publicKeyHash={account.publicKeyHash}
          tzwitter={tzwitter}
          onTweetClick={onTweetClick}
          feedKind={feedKind}
        />
      </div>
      <Popup isOpen={isOpen} onClose={onClose}>
        <div id="transfer-title">Transfer a tweet</div>
        <input
          id="transfer-input"
          placeholder="Destination: tz1...."
          value={form && form.destination}
          onChange={onDestinationChange}
        />
        <div id="transfer-footer">
          <button id="transfer-button" disabled={disabled} onClick={onTransfer}>
            Transfer
          </button>
        </div>
      </Popup>
    </div>
  );
};

export default Profile;
