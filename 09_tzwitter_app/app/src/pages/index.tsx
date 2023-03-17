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

const Error = () => {
  return <Navigate to="/" replace={true} />;
};

const secret = 'edsk3a5SDDdMWw3Q5hPiJwDXUosmZMTuKQkriPqY6UqtSfdLifpZbB';
const signer = new InMemorySigner(secret);
const TEZOS_URL = 'http://localhost:18731';
const ROLLUP_URL = 'http://localhost:8932';

const tezos = new TezosToolkit(TEZOS_URL);
tezos.setProvider({
  signer,
});

const tzwitterClient = new Tzwitter({ tezos, signer, rollupUrl: ROLLUP_URL });

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

  const router = createBrowserRouter([
    {
      path: '/',
      element: <Home tzwitter={tzwitter} />,
      errorElement: <Error />,
    },
    {
      path: '/feed/:publicKeyHash',
      element: <Feed tzwitter={tzwitter} />,
      errorElement: <Error />,
    },
    ...(account
      ? [
          {
            path: '/profile',
            element: <Profile tzwitter={tzwitter} account={account} />,
            errorElement: <Error />,
          },
        ]
      : []),
  ]);

  return <RouterProvider router={router} />;
};

export default Index;
