import './Input.css';

interface InputProperties {
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: () => void;
  disabled: boolean;
}

const Input = ({ value, onChange, onSubmit, disabled }: InputProperties) => {
  return (
    <div id="form">
      <input
        id="input"
        placeholder="What's happening?"
        onChange={onChange}
        value={value}
      ></input>
      <div id="buttons">
        <div></div>
        <button id="submit" onClick={onSubmit} disabled={disabled}>
          Tweet
        </button>
      </div>
    </div>
  );
};

export default Input;
