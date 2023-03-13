import { Tweet } from '../lib/tweet';
import './TweetComponent.css';

interface TweetProperty {
  tweet: Tweet;
  onLike: () => Promise<string>;
  onAuthorClick: () => void;
  onTweetClick?: () => void;
}

const TweetComponent = ({
  tweet,
  onLike,
  onAuthorClick,
  onTweetClick,
}: TweetProperty) => {
  const { id, author, content, likes } = tweet;
  const className: string = [
    'tweet',
    ...(onTweetClick ? ['clickable-tweet'] : []),
  ].join(' ');

  return (
    <div className={className} onClick={onTweetClick}>
      <div className="tweet-header">
        <div className="tweet-author" onClick={onAuthorClick}>
          {author}
        </div>
        <span className="tweet-header-separator">·</span>
        <div className="tweet-id">{id}</div>
      </div>
      <div className="tweet-content">{content}</div>
      <div className="tweet-footer">
        <div className="tweet-likes" onClick={onLike}>
          <img className="tweet-likes-icon" src={'/heart.svg'} alt="heart" />
          <span>{likes}</span>
        </div>
      </div>
    </div>
  );
};

export default TweetComponent;
