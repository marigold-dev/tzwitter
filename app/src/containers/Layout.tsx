import { useNavigate } from 'react-router-dom';
import './layout.css';
import Menu, { MenuEntries } from '../components/menu';
import { ReactNode } from 'react';

type LayoutProperty = {
  menu: MenuEntries;
  children: ReactNode;
  current: string;
};

const Layout = ({ menu, children, current }: LayoutProperty) => {
  const navigate = useNavigate();
  return (
    <div id="container">
      <Menu menu={menu} current={current} navigate={navigate} />
      <div id="content">{children}</div>
    </div>
  );
};

export default Layout;
