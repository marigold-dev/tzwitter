import { Tweet } from '../lib/tweet';
import TweetComponent from './TweetComponent';

interface FeedProperty {
  tweets: Array<Tweet>;
  onLike: (tweetId: number) => () => Promise<string>;
}

const Feed = ({ tweets, onLike }: FeedProperty) => {
  return (
    <div>
      {tweets.map((tweet) => (
        <TweetComponent
          key={tweet.id}
          tweet={tweet}
          onLike={onLike(tweet.id)}
        />
      ))}
    </div>
  );
};

export default Feed;
