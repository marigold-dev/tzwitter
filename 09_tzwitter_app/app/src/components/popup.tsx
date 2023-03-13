import './Popup.css';

interface PopupProperty {
  isOpen: boolean;
  onClose: () => void;
  children?: React.ReactNode;
}

const Popup = ({ isOpen, onClose, children }: PopupProperty) => {
  if (!isOpen) return <></>;
  return (
    <>
      <div id="popup-blur"></div>
      <div id="popup">
        <div id="popup-header">
          <img
            id="popup-header-cross"
            src={'/cancel.svg'}
            width="20px"
            height="20px"
            onClick={onClose}
            alt="cancel"
          />
        </div>
        <div id="popup-content">{children}</div>
      </div>
    </>
  );
};

export default Popup;
