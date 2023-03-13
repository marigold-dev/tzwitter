import "./FeedHeader.css";

interface FeedHeader {
    author: string,
    goToHome: () => void
}

const FeedHeader = ({ author, goToHome }: FeedHeader) => {
    return (
        <div className="feed-header">
            <div className="feed-header-go-home" onClick={goToHome}>{"<"}</div>
            <div className="feed-header-author">{author}</div>
        </div>);
}

export default FeedHeader