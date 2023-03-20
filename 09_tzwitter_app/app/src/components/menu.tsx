import './Menu.css';

export type MenuEntries = Array<{
  id: string;
  link: string;
  icon: string;
  text: string;
}>;

interface MenuType {
  current: string;
  navigate: (path: string) => void;
  menu: MenuEntries;
}

const Menu = ({ menu, current, navigate }: MenuType) => {
  return (
    <div id="menu">
      <div id={'menu-tezos'} onClick={() => navigate('/')}>
        <img
          src={'/tezos.png'}
          width={'30.54px'}
          height={'37.5px'}
          alt="tezos"
        />
      </div>
      {menu.map(({ id, link, icon, text }) => (
        <div key={id} className={'menu-entry'} onClick={() => navigate(link)}>
          <img src={icon} width={'30.54px'} height={'37.5px'} alt={id} />
          <div className={current === id ? 'menu-entry-bold' : undefined}>
            {text}
          </div>
        </div>
      ))}
    </div>
  );
};

export default Menu;
