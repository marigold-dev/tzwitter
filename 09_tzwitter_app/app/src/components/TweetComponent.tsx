import { Tweet } from '../lib/tweet';
import './TweetComponent.css';

const TweetComponent = (tweet: Tweet) => {
  const { id, author, content } = tweet;
  return (
    <div className="tweet">
      <div className="tweet-header">
        <div className="tweet-author">{author}</div>
        <span className="tweet-header-separator">·</span>
        <div className="tweet-id">{id}</div>
      </div>
      <div className="tweet-content">{content}</div>
    </div>
  );
};

export default TweetComponent;
