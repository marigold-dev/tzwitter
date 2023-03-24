import { Tweet } from '../lib/tweet';
import TweetComponent from './TweetComponent';

interface FeedProperty {
  tweets: Array<Tweet>;
  onLike?: (tweetId: number) => () => void;
  onAuthorClick?: (author: string) => () => void;
  onTransfer?: (tweetId: number) => () => void;
  onCollect?: (tweetId: number) => () => void;
}

const Feed = ({
  tweets,
  onLike,
  onAuthorClick,
  onTransfer,
  onCollect,
}: FeedProperty) => {
  return (
    <div>
      {tweets.map((tweet) => {
        return (
          <TweetComponent
            key={tweet.id}
            tweet={tweet}
            onLike={onLike && onLike(tweet.id)}
            onAuthorClick={onAuthorClick && onAuthorClick(tweet.author)}
            onTransfer={onTransfer && onTransfer(tweet.id)}
            onCollect={onCollect && onCollect(tweet.id)}
          />
        );
      })}
    </div>
  );
};

export default Feed;
