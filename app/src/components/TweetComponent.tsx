import { Tweet } from '../lib/tweet';
import './TweetComponent.css';

const TweetComponent = (tweet: Tweet) => {
  const { author, content } = tweet;
  return (
    <div className="tweet">
      <div className="tweet-author">{author}</div>
      <div className="tweet-content">{content}</div>
    </div>
  );
};

export default TweetComponent;
