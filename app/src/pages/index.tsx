import {
  Navigate,
  RouterProvider,
  createBrowserRouter,
} from 'react-router-dom';
import Home from './home';
import Account from './account';
import Profile from './profile';

const Error = () => {
  return <Navigate to="/" replace={true} />;
};

const Index = () => {
  const router = createBrowserRouter([
    {
      path: '/',
      element: <Home />,
      errorElement: <Error />,
    },
    {
      path: '/feed/:publicKeyHash',
      element: <Account />,
      errorElement: <Error />,
    },
    {
      path: '/profile',
      element: <Profile />,
      errorElement: <Error />,
    },
  ]);

  return <RouterProvider router={router} />;
};

export default Index;
