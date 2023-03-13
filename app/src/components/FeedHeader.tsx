import './FeedHeader.css';

interface FeedHeaderProperty {
  author: string;
  goToHome: () => void;
}

const FeedHeader = ({ author, goToHome }: FeedHeaderProperty) => {
  return (
    <div className="feed-header">
      <div className="feed-header-go-home" onClick={goToHome}>
        {'<'}
      </div>
      <div className="feed-header-author">{author}</div>
    </div>
  );
};

export default FeedHeader;
