import { Tweet } from '../lib/tweet';
import './TweetComponent.css';

interface TweetProperty {
  tweet: Tweet;
  onLike: () => Promise<string>;
  onTransfer?: () => void;
  onAuthorClick?: () => void;
}

const TweetComponent = ({
  tweet,
  onLike,
  onAuthorClick,
  onTransfer,
}: TweetProperty) => {
  const { id, author, content, likes } = tweet;
  const authorClassNames: string = [
    'tweet-author',
    ...(onAuthorClick ? ['clickable-tweet-author'] : []),
  ].join(' ');

  return (
    <div className={'tweet'}>
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
          className={'tweet-footer-buttom tweet-likes'}
          onClick={onLike}
          disabled={tweet.isLiked}
        >
          <img
            className="tweet-footer-icon tweet-likes-icon"
            src={tweet.isLiked ? '/heart-fill.svg' : '/heart.svg'}
            alt="heart"
          />
          <span>{likes}</span>
        </button>
        {onTransfer && (
          <button
            className={'tweet-footer-buttom tweet-transfer'}
            onClick={onTransfer}
          >
            <img
              className="tweet-footer-icon"
              src={'/transfer.svg'}
              alt="heart"
            />
          </button>
        )}
      </div>
    </div>
  );
};

export default TweetComponent;
