import {
  Navigate,
  RouterProvider,
  createBrowserRouter,
} from 'react-router-dom';
import Home from './home';
import Feed from './feed';
import Profile from './profile';
import { InMemorySigner } from '@taquito/signer';
import { Tzwitter } from '../lib/tzwitter';
import { useEffect, useState } from 'react';
import AccountType from '../lib/account';
import { TezosToolkit } from '@taquito/taquito';
import { ROLLUP_RPC, TEZOS_RPC } from '../config';

const Error = () => {
  return <Navigate to="/" replace={true} />;
};

const secret = 'edsk3a5SDDdMWw3Q5hPiJwDXUosmZMTuKQkriPqY6UqtSfdLifpZbB';
const signer = new InMemorySigner(secret);

const tezos = new TezosToolkit(TEZOS_RPC);
tezos.setProvider({
  signer,
});

const tzwitterClient = new Tzwitter({ tezos, signer, rollupUrl: ROLLUP_RPC });

const Index = () => {
  const [account, setAccount] = useState<AccountType | undefined>();
  const [tzwitter] = useState<Tzwitter>(tzwitterClient);

  useEffect(() => {
    // Later beacon sdk login logic should be put here
    // The provider of the tezos should be updated
    // Available pages should be filtered
    signer.publicKeyHash().then((publicKeyHash) => {
      setAccount({ publicKeyHash });
    });
  }, [tzwitter]);

  // Menu entries
  // Entries that need a connected user will be removed
  const menu = [
    {
      id: 'home',
      link: '/',
      text: 'Home',
      icon: '/home.svg',
      connected: false,
    },
    {
      id: 'profile',
      link: '/profile',
      text: 'Profile',
      icon: '/profile.svg',
      connected: true,
    },
  ].filter(({ connected }) => (connected && !account ? false : true));

  const router = createBrowserRouter([
    {
      path: '/',
      element: <Home menu={menu} tzwitter={tzwitter} />,
      errorElement: <Error />,
    },
    {
      path: '/feed/:publicKeyHash',
      element: <Feed menu={menu} tzwitter={tzwitter} />,
      errorElement: <Error />,
    },
    ...(account
      ? [
          {
            path: '/profile',
            element: (
              <Profile menu={menu} tzwitter={tzwitter} account={account} />
            ),
            errorElement: <Error />,
          },
        ]
      : []),
  ]);

  return <RouterProvider router={router} />;
};

export default Index;
