import { Tweet } from '../lib/tweet';
import TweetComponent from './TweetComponent';

interface FeedProperty {
  tweets: Array<Tweet>;
  onLike: (tweetId: number) => () => Promise<string>;
  onAuthorClick: (author: string) => () => void;
}

const Feed = ({ tweets, onLike, onAuthorClick }: FeedProperty) => {
  return (
    <div>
      {tweets.map((tweet) => (
        <TweetComponent
          key={tweet.id}
          tweet={tweet}
          onLike={onLike(tweet.id)}
          onAuthorClick={onAuthorClick(tweet.author)}
        />
      ))}
    </div>
  );
};

export default Feed;
