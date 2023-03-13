import './Menu.css';

type Entry = {
  id: string;
  className?: string;
  logo: {
    src: string;
    width: string;
    height: string;
  };
  text?: string;
  link: string;
};

const createMenu = (current: string, address: string): Array<Entry> =>
  [
    {
      id: 'menu-tezos',
      logo: {
        src: '/tezos.png',
        width: '30.54px',
        height: '37.5px',
      },
      link: '/',
    },
    {
      id: 'menu-home',
      logo: {
        src: '/home.svg',
        width: '25px',
        height: '25px',
      },
      text: 'Home',
      link: '/',
    },
    {
      id: 'menu-profile',
      logo: {
        src: '/profile.svg',
        width: '25px',
        height: '25px',
      },
      text: 'Profile',
      link: `/profile`,
    },
  ].map((entry) => {
    return entry.link === current
      ? { className: 'menu-entry-bold', ...entry }
      : entry;
  });

interface MenyType {
  current: string;
  navigate: (path: string) => void;
  address: string;
}

const Menu = ({ current, address, navigate }: MenyType) => {
  const menuEntries = createMenu(current, address);

  return (
    <div id="menu">
      {menuEntries.map(
        ({
          id,
          className,
          logo: { src, width, height },
          link,
          text,
        }: Entry) => {
          return (
            <div
              key={id}
              id={id}
              className={className}
              onClick={() => navigate(link)}
            >
              <img src={src} width={width} height={height} alt={id} />
              {text ? <div>{text}</div> : <></>}
            </div>
          );
        },
      )}
    </div>
  );
};

export default Menu;
