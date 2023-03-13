import { Tweet } from '../lib/tweet';
import TweetComponent from './TweetComponent';

interface FeedProperty {
  tweets: Array<Tweet>;
  onLike: (tweetId: number) => () => Promise<string>;
  onAuthorClick?: (author: string) => () => void;
  onTweetClick?: (tweetId: number) => () => void;
}

const Feed = ({
  tweets,
  onLike,
  onAuthorClick,
  onTweetClick,
}: FeedProperty) => {
  return (
    <div>
      {tweets.map((tweet) => {
        return (
          <TweetComponent
            key={tweet.id}
            tweet={tweet}
            onLike={onLike(tweet.id)}
            onAuthorClick={onAuthorClick && onAuthorClick(tweet.author)}
            onTweetClick={onTweetClick && onTweetClick(tweet.id)}
          />
        );
      })}
    </div>
  );
};

export default Feed;
