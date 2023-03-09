import { Tweet } from '../lib/tweet';
import TweetComponent from './TweetComponent';

const Feed = ({ tweets }: { tweets: Array<Tweet> }) => {
  return (
    <div>
      {tweets.map((tweet, id) => (
        <TweetComponent key={id} {...tweet} />
      ))}
    </div>
  );
};

export default Feed;
