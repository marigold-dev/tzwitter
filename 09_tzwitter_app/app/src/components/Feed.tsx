import { Tweet } from '../lib/tweet';
import TweetComponent from './TweetComponent';

const Feed = ({ tweets }: { tweets: Array<Tweet> }) => {
  return (
    <div>
      {tweets.map((tweet) => (
        <TweetComponent key={tweet.id} {...tweet} />
      ))}
    </div>
  );
};

export default Feed;
