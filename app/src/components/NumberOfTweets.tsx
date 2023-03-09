import './NumberOfTweets.css';

const NumberOfTweets = ({ number }: { number: Number }) => {
  return <div id="number-of-tweets">{number.toString()} Tweets</div>;
};

export default NumberOfTweets;
