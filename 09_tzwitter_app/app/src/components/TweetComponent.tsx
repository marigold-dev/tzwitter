import { Tweet } from '../lib/tweet';
import './TweetComponent.css';

interface TweetProperty {
  tweet: Tweet;
  onLike: () => Promise<string>;
  onAuthorClick?: () => void;
  onTweetClick?: () => void;
}

const TweetComponent = ({
  tweet,
  onLike,
  onAuthorClick,
  onTweetClick,
}: TweetProperty) => {
  const { id, author, content, likes } = tweet;
  const containerClassnames: string = [
    'tweet',
    ...(onTweetClick ? ['clickable-tweet'] : []),
  ].join(' ');

  const authorClassNames: string = [
    'tweet-author',
    ...(onAuthorClick ? ['clickable-tweet-author'] : []),
  ].join(' ');

  return (
    <div className={containerClassnames} onClick={onTweetClick}>
      <div className="tweet-header">
        <div className={authorClassNames} onClick={onAuthorClick}>
          {author}
        </div>
        <span className="tweet-header-separator">·</span>
        <div className="tweet-id">{id}</div>
      </div>
      <div className="tweet-content">{content}</div>
      <div className="tweet-footer">
        <button
          className={'tweet-likes'}
          onClick={onLike}
          disabled={tweet.isLiked}
        >
          <img
            className="tweet-likes-icon"
            src={tweet.isLiked ? '/heart-fill.svg' : '/heart.svg'}
            alt="heart"
          />
          <span>{likes}</span>
        </button>
      </div>
    </div>
  );
};

export default TweetComponent;
