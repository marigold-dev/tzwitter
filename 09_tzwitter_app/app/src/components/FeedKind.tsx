import './FeedKind.css';

interface FeedKindProperty {
  value: 'owned' | 'written';
  onChange: (value: 'owned' | 'written') => void;
}

const FeedKind = ({ value, onChange }: FeedKindProperty) => {
  const ownedClassName = [
    'owned',
    ...(value === 'owned' ? ['feed-kind-selected'] : []),
  ].join(' ');

  const writtenClassName = [
    'written',
    ...(value === 'written' ? ['feed-kind-selected'] : []),
  ].join(' ');

  return (
    <div id="feed-kind">
      <div
        id="owned"
        className={ownedClassName}
        onClick={() => onChange('owned')}
      >
        Owned
      </div>
      <div
        id="written"
        className={writtenClassName}
        onClick={() => onChange('written')}
      >
        Written
      </div>
    </div>
  );
};

export default FeedKind;
