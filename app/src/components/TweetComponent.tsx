import { Tweet } from '../lib/tweet';
import './TweetComponent.css';

interface TweetProperty {
  tweet: Tweet;
  onLike: () => Promise<string>;
}

const TweetComponent = ({ tweet, onLike }: TweetProperty) => {
  const { id, author, content, likes } = tweet;
  return (
    <div className="tweet">
      <div className="tweet-header">
        <div className="tweet-author">{author}</div>
        <span className="tweet-header-separator">·</span>
        <div className="tweet-id">{id}</div>
      </div>
      <div className="tweet-content">{content}</div>
      <div className="tweet-footer">
        <div className="tweet-likes" onClick={onLike}>
          <img className="tweet-likes-icon" src={'/heart.svg'} />
          <span>{likes}</span>
        </div>
      </div>
    </div>
  );
};

export default TweetComponent;
